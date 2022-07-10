use std::{ops::Deref, option};

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use reqwest::Request;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::models::user::User;

use super::get_current_user;

#[derive(Serialize, Deserialize)]
struct JwtBody {
    user_id: i64,
}

pub async fn get_user(connection: web::Data<SqlitePool>, req: HttpRequest) -> impl Responder {
    let optional_user = get_current_user(connection.as_ref(), &req).await;

    match optional_user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}
