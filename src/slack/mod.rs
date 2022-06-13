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

pub async fn post_message(channel: &str, text: &str) -> Result<(), ()> {
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

#[derive(Deserialize)]
struct ConversationsHistoryResponse {
  ok: bool,
  messages: Vec<SlackMessage>,
}

#[derive(Deserialize)]
pub struct SlackMessage {
  pub user: String,
  pub text: String,
  pub ts: String,
}

pub async fn get_messages(channel: &str, ts: &str, count: u32) -> Result<Vec<SlackMessage>, ()> {
  let client = reqwest::Client::new();
  let token = env::var("SLACK_TOKEN").unwrap_or_default();

  let result = client.get("https://slack.com/api/conversations.history")
    .query(&[("channel", channel), ("latest", ts), ("limit", count.to_string().as_ref()), ("inclusive", "true")])
    .bearer_auth(token)
    .send()
    .await;

  match result {
    Ok(resp) => {
      log::debug!("{:#?}", resp);
      let data = resp.json::<ConversationsHistoryResponse>()
        .await.map_err(|e| error!("{}", e))?;

      Ok(data.messages)
    }

    Err(e) => {
      error!("{}", e);
      Err(())
    }
  }
}

