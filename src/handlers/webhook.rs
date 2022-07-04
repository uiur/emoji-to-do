use std::{collections::HashMap, option};

use actix_web::{error::ErrorNotFound, post, web, HttpRequest, HttpResponse, Responder};
use futures::{future::try_join_all, try_join, TryFutureExt};
use log::{error, info};
use regex::{Captures, Regex};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::{
    github,
    models::{reaction::Reaction, team::Team, TeamConfigMap},
    slack::{self, SlackEvent, SlackItem, SlackMessage, SlackRequest},
};

pub async fn create_slack_events(
    data: web::Json<SlackRequest>,
    req: HttpRequest,
    connection: web::Data<SqlitePool>,
) -> actix_web::Result<impl Responder> {
    // Ignore duplicated requests due to http timeout
    if let Some(header_value) = req.headers().get("X-Slack-Retry-Reason") {
        if header_value.to_str().unwrap_or_default() == "http_timeout" {
            return Ok(HttpResponse::Ok().body(""));
        }
    }

    log::debug!("{:#?}", data);

    match data.0 {
        SlackRequest::UrlVerification { challenge } => Ok(HttpResponse::Ok().body(challenge)),

        SlackRequest::EventCallback { event } => match event {
            SlackEvent::ReactionAdded {
                user,
                reaction,
                item,
            } => handle_reaction_added(user, reaction, item, connection).await,

            SlackEvent::AppMention {
                user,
                channel,
                text,
            } => handle_app_mention(user, channel, text).await,

            _ => Ok(HttpResponse::Ok().body("")),
        },

        _ => Err(actix_web::error::ErrorBadRequest("")),
    }
}

async fn handle_reaction_added(
    user: String,
    reaction: String,
    item: SlackItem,
    connection: web::Data<SqlitePool>,
) -> actix_web::Result<HttpResponse> {
    let reactioner = slack::get_user_info(&user).await?;
    let team_id = reactioner.team_id;

    let team = Team::find(connection.as_ref(), &team_id)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?
        .ok_or(actix_web::error::ErrorNotFound("team is not found"))?;

    let record = Reaction::find_by_team_id_and_name(&connection, team.id, &reaction)
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

    if let Some(reaction_record) = record {
        log::info!("{:#?}", reaction_record);
        if let SlackItem::Message { channel, ts } = item {
            let messages = slack::get_messages(&channel, &ts, 3).await.map_err(|_| {
                actix_web::error::ErrorInternalServerError("failed to fetch slack messages")
            })?;

            let permalink = slack::get_permalink(&channel, &ts)
                .await
                .unwrap_or("".to_string());

            let users = try_join_all(
                messages
                    .iter()
                    .map(|message| slack::get_user_info(&message.user)),
            )
            .await?;

            let mut slack_user_map = HashMap::new();
            for user in &users {
                slack_user_map.insert(user.id.clone(), user.name.clone());
            }

            let text = messages
                .iter()
                .map(|message| {
                    let empty_username = "";
                    let username = users
                        .iter()
                        .find(|user| user.id == message.user)
                        .map(|user| user.name.as_str())
                        .unwrap_or(empty_username);
                    format!(
                        "{}: {}",
                        username,
                        humanize_slack_formatted_text(&message.text, &slack_user_map)
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");

            let title: String = messages
                .first()
                .and_then(|m| Some(String::from(&m.text)))
                .unwrap_or_default();

            let title = humanize_slack_formatted_text(&title, &slack_user_map);

            let body = format!("```\n{}\n```\n{}", &text, permalink);
            let issue = github::create_issue(&reaction_record.repo, &title, &body).await?;

            slack::post_message(
                &channel,
                &format!("<@{}> {}", reactioner.name, issue.html_url),
            )
            .await
            .map_err(|_| actix_web::error::ErrorInternalServerError(""))?;
        }
    }
    Ok(HttpResponse::Ok().body(""))
}

fn remove_head_mention(text: &str) -> String {
    let re = Regex::new(r"^<@[0-9A-Z]+>\s*").unwrap();
    re.replace(text, "").into()
}

fn humanize_slack_formatted_text(text: &str, slack_user_map: &HashMap<String, String>) -> String {
    let text = text.replace("\n", " ");
    let text = text.replace("`", "\\`");
    let re = Regex::new(r"<(?P<mark>[@#!])?(?P<a>.+?)(\|(?P<b>.+?))?>").unwrap();
    re.replace_all(&text, {
        |caps: &Captures| {
            if let Some(inner) = caps.name("a").and_then(|m| Some(m.as_str())) {
                match caps
                    .name("mark")
                    .and_then(|m| Some(m.as_str()))
                    .unwrap_or_default()
                {
                    "@" => {
                        let content = match slack_user_map.get(inner) {
                            Some(s) => s,
                            None => inner,
                        };

                        format!("@{}", content)
                    }

                    "!" => {
                        let content = match caps.name("b").and_then(|m| Some(m.as_str())) {
                            Some(b) => b,
                            None => inner,
                        };

                        format!("@{}", inner)
                    }

                    "#" => {
                        let content = match caps.name("b").and_then(|m| Some(m.as_str())) {
                            Some(b) => b,
                            None => inner,
                        };

                        format!("#{}", content)
                    }

                    _ => {
                        format!(
                            "{}{}",
                            caps.name("mark")
                                .and_then(|m| Some(m.as_str()))
                                .unwrap_or_default(),
                            inner
                        )
                    }
                }
            } else {
                "".to_string()
            }
        }
    })
    .into()
}

async fn handle_app_mention(
    user: String,
    channel: String,
    text: String,
) -> actix_web::Result<HttpResponse> {
    let content = remove_head_mention(&text);
    match content.as_str() {
        "ping" => {
            slack::post_message(&channel, "pong")
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError(""))?;
        }
        _ => {
            slack::post_message(&channel, &format!("```\n{}\n```", content))
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError(""))?;
        }
    }
    Ok(HttpResponse::Ok().body(""))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::slack;

    use super::{humanize_slack_formatted_text, remove_head_mention};

    #[test]
    fn test_remove_head_mention() {
        let text = remove_head_mention("<@U1234> ping");
        assert_eq!(text, "ping")
    }

    #[test]
    fn test_humanize_slack_formatted_text() {
        let mut slack_user_map = HashMap::new();
        slack_user_map.insert("U1234".to_string(), "uiur".to_string());

        let text = humanize_slack_formatted_text(
            "<@U1234> foo bar <https://github.com/uiur/sandbox/issues/1>",
            &slack_user_map,
        );
        assert_eq!(
            text,
            "@uiur foo bar https://github.com/uiur/sandbox/issues/1"
        );

        let text = humanize_slack_formatted_text("```\nfoo bar\n```", &slack_user_map);
        assert_eq!(text, "\\`\\`\\` foo bar \\`\\`\\`");

        let text = humanize_slack_formatted_text("<!here>", &slack_user_map);
        assert_eq!(text, "@here");

        let text = humanize_slack_formatted_text("<!subteam^SAZ94GDB8>", &slack_user_map);
        assert_eq!(text, "@subteam^SAZ94GDB8");

        let text = humanize_slack_formatted_text("<#C024BE7LR>", &slack_user_map);
        assert_eq!(text, "#C024BE7LR");
    }
}
