use actix_session::Session;
use actix_web::{Responder, HttpResponse, web};
use handlebars::{Handlebars, to_json};
use serde_json::json;
use sqlx::SqlitePool;

use crate::models::user::User;

pub async fn get_index(connection: web::Data<SqlitePool>, hb: web::Data<Handlebars<'_>>, session: Session) -> actix_web::Result<impl Responder> {
  let optional_user_id = session.get::<i64>("user_id")
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

  let mut user: Option<User> = None;
  if let Some(user_id) = optional_user_id {
    user = User::find(&connection, user_id).await
      .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
  }

  let data = json!({
    "name": "emoji-to-do",
    "user": user,
  });

  let body = hb.render("index", &data).unwrap();
  Ok(HttpResponse::Ok().body(body))
}
