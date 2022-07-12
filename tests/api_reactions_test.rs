use std::env;

use actix_web::cookie::{Cookie, CookieJar};
use hmac::{Hmac, Mac};
use jwt::{token::signed, SignWithKey};
use serde::Deserialize;
use serde_json::json;

mod test;

#[actix_rt::test]
async fn test_api_reactions() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;
    let client = reqwest::Client::new();

    let user_id =
        emoji_to_do::models::user::User::create(&connection, "TEAM", "USER", "TOKEN").await?;
    let token = emoji_to_do::token::generate(user_id)?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", "TEAM").await?;

    emoji_to_do::models::reaction::Reaction::create(&connection, team_id, "eyes", "uiur/sandbox")
        .await?;

    let response = client
        .get(format!("{}/api/teams/{}/reactions", host, team_id))
        .header("Authorization", format!("Bearer {}", token))
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
    let client = reqwest::Client::new();

    let user_id =
        emoji_to_do::models::user::User::create(&connection, "TEAM1", "USER", "TOKEN").await?;
    let token = emoji_to_do::token::generate(user_id)?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", "TEAM2").await?;

    let reaction_id = emoji_to_do::models::reaction::Reaction::create(
        &connection,
        team_id,
        "eyes",
        "uiur/sandbox",
    )
    .await?;

    let response = client
        .get(format!("{}/api/teams/{}/reactions", host, team_id))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 404);

    Ok(())
}
