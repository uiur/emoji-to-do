use std::env;

use actix_web::cookie::{Cookie, CookieJar};
use hmac::{Hmac, Mac};
use jwt::{token::signed, SignWithKey};
use serde_json::json;

mod test;

fn generate_jwt(user_id: i64) -> Result<String, Box<dyn std::error::Error>> {
    let master_key = env::var("MASTER_KEY").expect("MASTER_KEY is expected");
    let key: Hmac<sha2::Sha256> = Hmac::new_from_slice(master_key.as_bytes())?;
    let body = json!({ "user_id": user_id });
    Ok(body.sign_with_key(&key).unwrap())
}

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

    let user_id =
        emoji_to_do::models::user::User::create(&connection, "TEAM", "USER", "TOKEN").await?;
    let token = generate_jwt(user_id)?;

    let response = client
        .get(format!("{}/api/user", host))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status().as_u16(), 200);

    let body = response.text().await.expect("failed to fetch body");

    println!("{}", body);
    Ok(())
}
