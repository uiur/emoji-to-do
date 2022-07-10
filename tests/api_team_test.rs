use std::env;

use actix_web::cookie::{Cookie, CookieJar};
use hmac::{Hmac, Mac};
use jwt::{token::signed, SignWithKey};
use serde_json::json;

mod test;

#[actix_rt::test]
async fn test_api_team_when_authenticated() -> Result<(), Box<dyn std::error::Error>> {
    let (host, connection) = test::spawn_app().await;
    let client = reqwest::Client::new();

    let user_id =
        emoji_to_do::models::user::User::create(&connection, "TEAM", "USER", "TOKEN").await?;
    let token = emoji_to_do::token::generate(user_id)?;
    let team_id =
      emoji_to_do::models::team::Team::create(&connection, "TEAM EMOJI", "TEAM").await?;

    let response = client
        .get(format!("{}/api/team", host))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.expect("failed to fetch body");

    println!("{}", body);
    Ok(())
}
