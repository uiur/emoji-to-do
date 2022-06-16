use actix_web::{Responder, HttpResponse, post, web, HttpRequest};
use futures::{try_join, future::try_join_all, TryFutureExt};
use log::{info, error};
use serde::Deserialize;

use crate::{slack::{SlackRequest, SlackEvent, SlackItem, SlackMessage, self}, github, models::TeamConfigMap, AppState};

#[post("/webhook/slack/events")]
pub async fn create_slack_events(data: web::Json<SlackRequest>, req: HttpRequest, team_config_map: web::Data<TeamConfigMap>) -> actix_web::Result<impl Responder> {
    // Ignore duplicated requests due to http timeout
    if let Some(header_value) = req.headers().get("X-Slack-Retry-Reason") {
      if header_value.to_str().unwrap_or_default() == "http_timeout" {
        return Ok(HttpResponse::Ok().body(""))
      }
    }

    info!("{:#?}", data);

    match data.0 {
        SlackRequest::UrlVerification { challenge } => {
            Ok(HttpResponse::Ok().body(challenge))
        }

        SlackRequest::EventCallback { event } => {
          match event {
            SlackEvent::ReactionAdded { user, reaction, item } => {
              let reactioner = slack::get_user_info(&user).await?;

              let team_id = reactioner.team_id;
              let pattern = team_config_map.find(&team_id, "", &reaction).unwrap();

              if let SlackItem::Message { channel, ts } = item {
                let messages = slack::get_messages(&channel, &ts, 3).await
                  .map_err(|_| actix_web::error::ErrorInternalServerError("failed to fetch slack messages"))?;

                let permalink = slack::get_permalink(&channel, &ts).await.unwrap_or("".to_string());

                let users = try_join_all(
                  messages.iter()
                    .map(|message| slack::get_user_info(&message.user))
                ).await?;


                let text = messages.iter().map(|message| {
                  let empty_username = "";
                  let username = users.iter().find(|user| user.id == message.user).map(|user| user.name.as_str()).unwrap_or(empty_username);
                  format!("{}: {}", username, message.text)
                }).collect::<Vec<String>>().join("\n");

                info!("{}\n{}", text, permalink);
                let title: String = messages.first().and_then(|m| Some(String::from(&m.text))).unwrap_or_default();

                let body = format!("```\n{}\n```\n{}", text, permalink);
                let issue = github::create_issue(&pattern.repo, &title, &body).await?;

                slack::post_message(&channel, &format!("<@{}> {}", reactioner.name, issue.html_url)).await
                  .map_err(|_| actix_web::error::ErrorInternalServerError(""))?;
              }

              Ok(HttpResponse::Ok().body(""))
            }

            _ => {
              Ok(HttpResponse::Ok().body(""))
            }
          }
        }

        _ => {
            Err(actix_web::error::ErrorBadRequest(""))
        }
    }
}
