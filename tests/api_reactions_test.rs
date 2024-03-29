#![feature(assert_matches)]
use std::assert_matches::assert_matches;

use emoji_to_do::entities;

use sea_orm::{EntityTrait, ModelTrait, Set};
use serde::Deserialize;
use serde_json::json;

use test::{create_api_client, create_user};

mod test;

#[actix_rt::test]
async fn test_api_reactions() -> Result<(), Box<dyn std::error::Error>> {
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

    let _reaction_assignee_id =
        entities::reaction_assignee::Entity::insert(entities::reaction_assignee::ActiveModel {
            reaction_id: Set(reaction_id),
            name: Set("uiur".to_owned()),
            ..Default::default()
        })
        .exec(&connection)
        .await?
        .last_insert_id;

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
    let team_id = entities::team::Entity::insert(entities::team::ActiveModel {
        name: Set("TEAM EMOJI".to_owned()),
        slack_team_id: Set("FOOBAR".to_owned()),
        ..Default::default()
    })
    .exec(&connection)
    .await?
    .last_insert_id;

    let _reaction_id = entities::reaction::Entity::insert(entities::reaction::ActiveModel {
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
        .get(format!("{}/api/teams/{}/reactions", host, team_id))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 404);

    Ok(())
}

#[derive(Deserialize)]
struct CreateReactionResponse {
    id: i32,
}

#[actix_rt::test]
async fn test_api_create_reaction() -> Result<(), Box<dyn std::error::Error>> {
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

    let client = create_api_client(user.id)?;
    let response = client
        .post(format!("{}/api/teams/{}/reactions", host, team_id))
        .json(&json!({
                  "name": "eyes",
                  "repo": "uiur/sandbox",
                  "reaction_assignees": [
                    { "name": "uiur" }
                  ]
        }))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 201);
    let json: CreateReactionResponse = response.json().await?;
    let id: i32 = json.id;
    let optional_reaction = entities::prelude::Reaction::find_by_id(id)
        .one(&connection)
        .await?;
    assert_matches!(optional_reaction, Some(_));

    if let Some(reaction) = optional_reaction {
        let reaction_assignees = reaction
            .find_related(entities::prelude::ReactionAssignee)
            .all(&connection)
            .await?;
        assert_eq!(reaction_assignees.len(), 1);
    }

    Ok(())
}

#[actix_rt::test]
async fn test_api_update_reaction() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;
    let user = create_user(&connection).await?;
    let team_id = entities::team::Entity::insert(entities::team::ActiveModel {
        name: Set("team_name".to_owned()),
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
        .put(format!("{}/api/reactions/{}", host, reaction_id))
        .json(&json!({
          "name": "eyes",
          "repo": "uiur/sandbox2",
          "reaction_assignees": [
            {"name": "uiur"}
          ]
        }))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 200);

    let reaction = entities::prelude::Reaction::find_by_id(reaction_id)
        .one(&connection)
        .await?
        .unwrap();
    assert_eq!(reaction.repo, "uiur/sandbox2");

    Ok(())
}

#[actix_rt::test]
async fn test_api_destroy_reaction() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;
    let user = create_user(&connection).await?;
    let team_id = entities::team::Entity::insert(entities::team::ActiveModel {
        name: Set("team_name".to_owned()),
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
        .delete(format!("{}/api/reactions/{}", host, reaction_id))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 204);

    let optional_reaction = entities::prelude::Reaction::find_by_id(reaction_id)
        .one(&connection)
        .await?;
    assert_matches!(optional_reaction, None);

    Ok(())
}
