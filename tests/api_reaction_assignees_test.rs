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

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Deserialize)]
struct CreateReactionAssigneeResponse {
    id: i64,
}

#[actix_rt::test]
async fn test_api_create_reaction_assignee() -> TestResult {
    let (host, connection) = test::spawn_app().await;

    let user = create_user(&connection).await?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", &user.slack_team_id)
            .await?;

    let reaction_id = Reaction::create(&connection, team_id, "eyes", "uiur/sandbox").await?;
    let client = create_api_client(user.id)?;
    let response = client
        .post(format!(
            "{}/api/reactions/{}/reaction_assignees",
            host, reaction_id
        ))
        .json(&json!({
                  "name": "uiur"
        }))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 201);
    let json: CreateReactionAssigneeResponse = response.json().await?;
    let id: i64 = json.id;
    let optional_reaction_assignee = ReactionAssignee::find(&connection, id).await?;
    assert_matches!(optional_reaction_assignee, Some(_));

    Ok(())
}

#[actix_rt::test]
async fn test_api_destroy_reaction_assignee() -> TestResult {
    let (host, connection) = test::spawn_app().await;
    let user = create_user(&connection).await?;
    let team_id =
        emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", &user.slack_team_id)
            .await?;

    let reaction_id = Reaction::create(&connection, team_id, "eyes", "uiur/sandbox").await?;
    let reaction_assignee_id = ReactionAssignee::create(&connection, reaction_id, "uiur").await?;
    let response = create_api_client(user.id)?
        .delete(format!(
            "{}/api/reaction_assignees/{}",
            host, reaction_assignee_id
        ))
        .send()
        .await
        .expect("failed to execute api");

    assert_eq!(response.status(), 204);

    let optional_reaction_assignee =
        ReactionAssignee::find(&connection, reaction_assignee_id).await?;
    assert_matches!(optional_reaction_assignee, None);

    Ok(())
}
