#![feature(assert_matches)]
use std::{assert_matches::assert_matches, collections::HashMap, env, option};

use actix_web::cookie::{Cookie, CookieJar};
use emoji_to_do::models::{
    reaction::Reaction, reaction_assignee::ReactionAssignee, team::Team, user::User,
};
use hmac::{Hmac, Mac};
use jwt::{token::signed, SignWithKey};
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;
use test::{create_api_client, create_user};

mod test;

#[actix_rt::test]
async fn test_api_reactions() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;

    let user = create_user(&connection).await?;
    let team_id = Team::create(&connection, "TEAM EMOJI", &user.slack_team_id).await?;

    let reaction_id = Reaction::create(&connection, team_id, "eyes", "uiur/sandbox").await?;
    ReactionAssignee::create(&connection, reaction_id, "uiur").await?;

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

#[derive(Deserialize)]
struct CreateReactionResponse {
    id: i64,
}

#[actix_rt::test]
async fn test_api_create_reaction() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;

    let user = create_user(&connection).await?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", &user.slack_team_id)
            .await?;

    let client = create_api_client(user.id)?;
    let response = client
        .post(format!("{}/api/teams/{}/reactions", host, team_id))
        .json(&json!({
                  "name": "eyes", "repo": "uiur/sandbox"
        }))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 201);
    let json: CreateReactionResponse = response.json().await?;
    let id: i64 = json.id;
    let optional_reaction = Reaction::find(&connection, id).await?;
    assert_matches!(optional_reaction, Some(_));

    Ok(())
}

#[actix_rt::test]
async fn test_api_update_reaction() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;
    let user = create_user(&connection).await?;
    let team_id = Team::create(&connection, "team_name", &user.slack_team_id).await?;

    let reaction_id = Reaction::create(&connection, team_id, "eyes", "uiur/sandbox").await?;

    let client = create_api_client(user.id)?;
    let response = client
        .put(format!("{}/api/reactions/{}", host, reaction_id))
        .json(&json!({
          "name": "eyes",
          "repo": "uiur/sandbox2"
        }))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 200);

    let reaction = Reaction::find(&connection, reaction_id).await?.unwrap();
    assert_eq!(reaction.repo, "uiur/sandbox2");

    Ok(())
}

#[actix_rt::test]
async fn test_api_destroy_reaction() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;
    let user = create_user(&connection).await?;
    let team_id = Team::create(&connection, "team_name", &user.slack_team_id).await?;

    let reaction_id = Reaction::create(&connection, team_id, "eyes", "uiur/sandbox").await?;
    let client = create_api_client(user.id)?;
    let response = client
        .delete(format!("{}/api/reactions/{}", host, reaction_id))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 204);

    let optional_reaction = Reaction::find(&connection, reaction_id).await?;
    assert_matches!(optional_reaction, None);

    Ok(())
}
