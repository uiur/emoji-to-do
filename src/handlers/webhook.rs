use std::{env, collections::HashMap};

use actix_web::{Responder, HttpResponse, post, web};
use log::{info, error};
use serde::Deserialize;

use crate::slack::{SlackRequest, SlackEvent, SlackItem, post_message};

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
                post_message(&channel, &format!(":{}:", reaction)).await
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
