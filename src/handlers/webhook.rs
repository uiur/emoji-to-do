use std::{env, collections::HashMap};

use actix_web::{Responder, HttpResponse, App, HttpServer, get, post, middleware::Logger, web};
use log::{info, error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SlackRequest {
    // https://api.slack.com/events/url_verification
    UrlVerification { challenge: String },

    EventCallback { event: SlackEvent },

    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SlackEvent {
  // https://api.slack.com/events/reaction_added
  ReactionAdded { user: String, reaction: String, item: SlackItem },

  #[serde(other)]
  Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SlackItem {
  Message { channel: String, ts: String },

  #[serde(other)]
  Other
}

async fn slack_post_message(channel: &str, text: &str) -> Result<(), ()> {
  let client = reqwest::Client::new();
  let token = env::var("SLACK_TOKEN").unwrap_or_default();

  let mut data = HashMap::new();
  data.insert("channel", channel);
  data.insert("text", text);

  let resp = client.post("https://slack.com/api/chat.postMessage")
    .header("Content-Type", "application/json")
    .bearer_auth(token)
    .json(&data)
    .send()
    .await.map_err(|e| ())?;

  Ok(())
}

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
                slack_post_message(&channel, &format!(":{}:", reaction)).await
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
