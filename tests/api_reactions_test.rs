use std::{collections::HashMap, env};

use actix_web::cookie::{Cookie, CookieJar};
use emoji_to_do::models::user::User;
use hmac::{Hmac, Mac};
use jwt::{token::signed, SignWithKey};
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;

mod test;

fn create_api_client(user_id: i64) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let token = emoji_to_do::token::generate(user_id)?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

async fn create_user(connection: &SqlitePool) -> Result<User, Box<dyn std::error::Error>> {
    let user_id =
        emoji_to_do::models::user::User::create(connection, "TEAM", "USER", "TOKEN").await?;
    let user = emoji_to_do::models::user::User::find(connection, user_id)
        .await?
        .unwrap();
    Ok(user)
}

#[actix_rt::test]
async fn test_api_reactions() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;

    let user = create_user(&connection).await?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", &user.slack_team_id)
            .await?;

    emoji_to_do::models::reaction::Reaction::create(&connection, team_id, "eyes", "uiur/sandbox")
        .await?;

    let client = create_api_client(user.id)?;
    let response = client
        .get(format!("{}/api/teams/{}/reactions", host, team_id))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.expect("failed to fetch body");
    println!("{}", body);

    let values: Vec<serde_json::Value> = serde_json::from_str(&body)?;
    assert_eq!(values[0]["id"], team_id);

    Ok(())
}

#[actix_rt::test]
async fn test_api_reactions_when_user_does_not_belong_to_team(
) -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;

    let user = create_user(&connection).await?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", "TEAM2").await?;

    let reaction_id = emoji_to_do::models::reaction::Reaction::create(
        &connection,
        team_id,
        "eyes",
        "uiur/sandbox",
    )
    .await?;

    let client = create_api_client(user.id)?;

    let response = client
        .get(format!("{}/api/teams/{}/reactions", host, team_id))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 404);

    Ok(())
}

#[actix_rt::test]
async fn test_api_create_reaction() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;

    let user = create_user(&connection).await?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", &user.slack_team_id)
            .await?;

    let request_body = HashMap::from([("name", "eyes"), ("repo", "uiur/sandbox")]);

    let client = create_api_client(user.id)?;
    let response = client
        .post(format!("{}/api/teams/{}/reactions", host, team_id))
        .json(&request_body)
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 201);

    Ok(())
}
