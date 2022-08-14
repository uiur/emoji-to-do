use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::entities;

pub mod reaction;
pub mod reaction_assignee;
pub mod team;
pub mod token;
pub mod user;

#[derive(Serialize, Deserialize)]
struct JwtBody {
    user_id: i32,
}

pub async fn get_current_user(
    connection: &DatabaseConnection,
    req: &HttpRequest,
) -> Option<entities::user::Model> {
    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY is expected");
    let key: Hmac<sha2::Sha256> = Hmac::new_from_slice(master_key.as_bytes()).unwrap();

    let optional_authorization = req
        .headers()
        .get("Authorization")
        .and_then(|v| Some(v.to_str().unwrap_or_default().to_owned()));

    match optional_authorization {
        Some(authorization) => {
            let a: Vec<&str> = authorization.split(' ').collect();
            let token = a[1];
            let data: JwtBody = token.verify_with_key(&key).unwrap();
            entities::prelude::User::find_by_id(data.user_id)
                .one(connection)
                .await
                .unwrap_or(None)
        }
        None => None,
    }
}
