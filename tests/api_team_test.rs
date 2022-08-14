use emoji_to_do::entities;
use hmac::Mac;

use sea_orm::{EntityTrait, Set};
use serde::Deserialize;

mod test;

#[derive(Deserialize)]
struct AssertedTeamResponse {
    id: i32,
    name: String,
    slack_team_id: String,
}

#[actix_rt::test]
async fn test_api_team_when_authenticated() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;
    let client = reqwest::Client::new();

    let user_id = entities::user::Entity::insert(entities::user::ActiveModel {
        slack_team_id: Set("TEAM".to_owned()),
        slack_user_id: Set("USER".to_owned()),
        slack_token: Set("TOKEN".to_owned()),
        ..Default::default()
    })
    .exec(&connection)
    .await?
    .last_insert_id;

    let token = emoji_to_do::token::generate(user_id)?;
    let user = entities::prelude::User::find_by_id(user_id)
        .one(&connection)
        .await?
        .expect("user is not found");

    let team_id = entities::team::Entity::insert(entities::team::ActiveModel {
        name: Set("TEAM EMOJI".to_owned()),
        slack_team_id: Set(user.slack_team_id),
        ..Default::default()
    })
    .exec(&connection)
    .await?
    .last_insert_id;

    let response = client
        .get(format!("{}/api/team", host))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.expect("failed to fetch body");

    println!("{}", body);

    let value: AssertedTeamResponse = serde_json::from_str(&body)?;
    assert_eq!(value.id, team_id);

    Ok(())
}
