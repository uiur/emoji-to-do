use std::{collections::HashMap, env};

use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Issue {
    pub html_url: String,
}

#[derive(Debug)]
pub enum GithubClientError {
    ApiError,
    JsonError,
}

impl std::fmt::Display for GithubClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GithubClientError::ApiError => write!(f, "github returned api error"),
            GithubClientError::JsonError => write!(f, "github returned json error"),
        }
    }
}
impl std::error::Error for GithubClientError {}

pub async fn create_issue(
    repo: &str,
    title: &str,
    body: &str,
) -> Result<Issue, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN").unwrap_or_default();

    let client = reqwest::Client::new();

    let mut params = HashMap::new();
    params.insert("title", title);
    params.insert("body", body);

    let resp = client
        .post(format!("https://api.github.com/repos/{}/issues", repo))
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "uiur/emoji-to-do")
        .bearer_auth(token)
        .json(&params)
        .send()
        .await
        .map_err(|_e| GithubClientError::ApiError)?;

    log::debug!("{:#?}", resp);
    if !resp.status().is_success() {
        log::error!("{:#?}", resp.text().await?);
        return Err(GithubClientError::ApiError.into());
    }

    let issue = resp
        .json::<Issue>()
        .await
        .map_err(|_e| GithubClientError::JsonError)?;

    Ok(issue)
    // Err(GithubClientError::ApiError.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserInstallationResponse {
    installations: Vec<Installation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Installation {
    pub id: i32,
}

pub async fn list_user_installations(
    token: &str,
) -> Result<Vec<Installation>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.github.com/user/installations")
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "uiur/emoji-to-do")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_e| GithubClientError::ApiError)?;

    log::debug!("{:#?}", resp);
    let result = resp.json::<ListUserInstallationResponse>().await?;
    Ok(result.installations)
}
