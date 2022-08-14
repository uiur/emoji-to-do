use std::env;

use actix_web::cookie::{Cookie, CookieJar};
use emoji_to_do::entities;
use hmac::{Hmac, Mac};
use jwt::{token::signed, SignWithKey};
use sea_orm::{EntityTrait, Set};
use serde_json::json;

mod test;

#[actix_rt::test]
async fn test_api_user_when_not_authenticated() {
    let (host, _) = test::spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/user", host))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 404);
}

#[actix_rt::test]
async fn test_api_user_when_authenticated() -> Result<(), Box<dyn std::error::Error>> {
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

    let response = client
        .get(format!("{}/api/user", host))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.expect("failed to fetch body");

    println!("{}", body);

    let value: serde_json::Value = serde_json::from_str(&body)?;
    assert_eq!(value["id"], user_id);

    Ok(())
}
