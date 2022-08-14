#![feature(assert_matches)]
use std::{assert_matches::assert_matches, collections::HashMap, env, option};

use actix_web::cookie::{Cookie, CookieJar};
use emoji_to_do::{
    entities,
    models::{reaction::Reaction, reaction_assignee::ReactionAssignee, team::Team, user::User},
};
use hmac::{Hmac, Mac};
use jwt::{token::signed, SignWithKey};
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;
use serde_json::json;
use sqlx::SqlitePool;
use test::{create_api_client, create_user};

mod test;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Deserialize)]
struct CreateReactionAssigneeResponse {
    id: i32,
}

#[actix_rt::test]
async fn test_api_create_reaction_assignee() -> TestResult {
    let (host, connection) = test::spawn_app().await;

    let user = create_user(&connection).await?;
    let team_id = entities::team::Entity::insert(entities::team::ActiveModel {
        name: Set("TEAM EMOJI".to_owned()),
        slack_team_id: Set(user.slack_team_id),
        ..Default::default()
    })
    .exec(&connection)
    .await?
    .last_insert_id;

    let reaction_id = entities::reaction::Entity::insert(entities::reaction::ActiveModel {
        team_id: Set(team_id),
        name: Set("eyes".to_owned()),
        repo: Set("uiur/sandbox".to_owned()),
        ..Default::default()
    })
    .exec(&connection)
    .await?
    .last_insert_id;

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
    let id: i32 = json.id;
    let optional_reaction_assignee = entities::prelude::ReactionAssignee::find_by_id(id)
        .one(&connection)
        .await?;
    assert_matches!(optional_reaction_assignee, Some(_));

    Ok(())
}

#[actix_rt::test]
async fn test_api_destroy_reaction_assignee() -> TestResult {
    let (host, connection) = test::spawn_app().await;
    let user = create_user(&connection).await?;
    let team_id = entities::team::Entity::insert(entities::team::ActiveModel {
        name: Set("TEAM EMOJI".to_owned()),
        slack_team_id: Set(user.slack_team_id),
        ..Default::default()
    })
    .exec(&connection)
    .await?
    .last_insert_id;

    let reaction_id = entities::reaction::Entity::insert(entities::reaction::ActiveModel {
        team_id: Set(team_id),
        name: Set("eyes".to_owned()),
        repo: Set("uiur/sandbox".to_owned()),
        ..Default::default()
    })
    .exec(&connection)
    .await?
    .last_insert_id;

    let reaction_assignee_id =
        entities::reaction_assignee::Entity::insert(entities::reaction_assignee::ActiveModel {
            reaction_id: Set(reaction_id),
            name: Set("uiur".to_owned()),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id;

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
        entities::prelude::ReactionAssignee::find_by_id(reaction_assignee_id)
            .one(&connection)
            .await?;
    assert_matches!(optional_reaction_assignee, None);

    Ok(())
}
