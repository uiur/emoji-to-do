use std::{env, collections::HashMap};

use actix_web::{Responder, HttpResponse, post, web};
use futures::{try_join, future::try_join_all, TryFutureExt};
use log::{info, error};
use serde::Deserialize;

use crate::slack::{SlackRequest, SlackEvent, SlackItem, SlackMessage, self};

#[post("/webhook/slack/events")]
pub async fn create_slack_events(data: web::Json<SlackRequest>) -> actix_web::Result<impl Responder> {
    info!("{:#?}", data);

    match data.0 {
        SlackRequest::UrlVerification { challenge } => {
            Ok(HttpResponse::Ok().body(challenge))
        }

        SlackRequest::EventCallback { event } => {
          match event {
            SlackEvent::ReactionAdded { user, reaction, item } => {
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

                info!("{}\n{}", text, permalink)

                //   text.push_str(format!("{}: {}\n", message_author.name, message.text).as_str());
                  // slack::post_message(&channel, &format!("{}: :{}:\n{}: {}\n{}", author.name, reaction, message_author.name, message.text, permalink)).await
                  //   .map_err(|_| actix_web::error::ErrorInternalServerError(""))?;
                // }
                // info!("{}", text);


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
