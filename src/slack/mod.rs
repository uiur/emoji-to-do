use std::{collections::HashMap, env};

use log::error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SlackRequest {
    // https://api.slack.com/events/url_verification
    UrlVerification {
        challenge: String,
    },

    EventCallback {
        event: SlackEvent,
    },

    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SlackEvent {
    // https://api.slack.com/events/reaction_added
    ReactionAdded {
        user: String,
        reaction: String,
        item: SlackItem,
    },
    AppMention {
        user: String,
        text: String,
        channel: String,
    },

    #[serde(other)]
    Other,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SlackItem {
    Message {
        channel: String,
        ts: String,
    },

    #[serde(other)]
    Other,
}

pub async fn post_message(channel: &str, text: &str) -> Result<(), ()> {
    let client = reqwest::Client::new();
    let token = env::var("SLACK_TOKEN").unwrap_or_default();

    let mut data = HashMap::new();
    data.insert("channel", channel);
    data.insert("text", text);

    let _resp = client
        .post("https://slack.com/api/chat.postMessage")
        .header("Content-Type", "application/json")
        .bearer_auth(token)
        .json(&data)
        .send()
        .await
        .map_err(|_e| ())?;

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

    let result = client
        .get("https://slack.com/api/conversations.history")
        .query(&[
            ("channel", channel),
            ("latest", ts),
            ("limit", count.to_string().as_ref()),
            ("inclusive", "true"),
        ])
        .bearer_auth(token)
        .send()
        .await;

    match result {
        Ok(resp) => {
            log::debug!("{:#?}", resp);
            let data = resp
                .json::<ConversationsHistoryResponse>()
                .await
                .map_err(|e| error!("{}", e))?;

            Ok(data.messages)
        }

        Err(e) => {
            error!("{}", e);
            Err(())
        }
    }
}

#[derive(Deserialize)]
struct UserInfoResponse {
    ok: bool,
    user: SlackUser,
}

#[derive(Deserialize)]
pub struct SlackUser {
    pub id: String,
    pub name: String,
    pub team_id: String,
}

#[derive(Debug)]
enum SlackClientError {
    ApiError,
    JsonError,
}

impl std::fmt::Display for SlackClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SlackClientError::ApiError => write!(f, "api error"),
            SlackClientError::JsonError => write!(f, "json parse error"),
        }
    }
}

impl std::error::Error for SlackClientError {}

pub async fn get_user_info(user: &str) -> Result<SlackUser, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = env::var("SLACK_TOKEN").unwrap_or_default();
    let result = client
        .get("https://slack.com/api/users.info")
        .query(&[("user", user)])
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_e| SlackClientError::ApiError)?
        .json::<UserInfoResponse>()
        .await
        .map_err(|_e| SlackClientError::JsonError)?;

    Ok(result.user)
}

#[derive(Deserialize)]
struct GetPermalinkResponse {
    permalink: String,
}

pub async fn get_permalink(channel: &str, ts: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let token = env::var("SLACK_TOKEN").unwrap_or_default();
    let data = client
        .get("https://slack.com/api/chat.getPermalink")
        .query(&[("channel", channel), ("message_ts", ts)])
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_e| SlackClientError::ApiError)?
        .json::<GetPermalinkResponse>()
        .await
        .map_err(|_e| SlackClientError::JsonError)?;

    Ok(data.permalink)
}
