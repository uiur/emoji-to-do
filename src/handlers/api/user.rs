use std::{ops::Deref, option};

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use reqwest::Request;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::models::user::User;

#[derive(Serialize, Deserialize)]
struct JwtBody {
    user_id: i64,
}

async fn get_current_user(connection: &SqlitePool, req: &HttpRequest) -> Option<User> {
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
            User::find(connection, data.user_id).await.unwrap_or(None)
        }
        None => None,
    }
}

pub async fn get_user(connection: web::Data<SqlitePool>, req: HttpRequest) -> impl Responder {
    let optional_user = get_current_user(connection.as_ref(), &req).await;

    match optional_user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}
