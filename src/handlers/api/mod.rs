use actix_session::{Session, SessionExt};
use actix_web::HttpRequest;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::entities;

use self::user::get_user;

pub mod reaction;
pub mod reaction_assignee;
pub mod team;
pub mod token;
pub mod user;

#[derive(Serialize, Deserialize)]
struct JwtBody {
    user_id: i32,
}

pub fn get_user_id_from_authorization_header(req: &HttpRequest) -> Option<i32> {
    let authorization = req
        .headers()
        .get("Authorization")
        .and_then(|v| Some(v.to_str().unwrap_or_default().to_owned()))?;

    let master_key = std::env::var("MASTER_KEY").expect("MASTER_KEY is expected");
    let key: Hmac<sha2::Sha256> = Hmac::new_from_slice(master_key.as_bytes()).unwrap();

    let a: Vec<&str> = authorization.split(' ').collect();
    let token = a[1];
    let data: JwtBody = token.verify_with_key(&key).unwrap();

    Some(data.user_id)
}

pub async fn get_current_user(
    connection: &DatabaseConnection,
    req: &HttpRequest,
) -> Option<entities::user::Model> {
    let session = req.get_session();

    let user_id = session
        .get::<i32>("user_id")
        .ok()?
        .or_else(|| get_user_id_from_authorization_header(&req))?;

    entities::prelude::User::find_by_id(user_id)
        .one(connection)
        .await
        .unwrap_or(None)
}
