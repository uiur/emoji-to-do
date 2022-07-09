use actix_session::Session;
use actix_web::{Responder, HttpResponse, web};
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_index(hb: web::Data<Handlebars<'_>>, session: Session) -> actix_web::Result<impl Responder> {
  let user_id = session.get::<i32>("user_id")
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

  let data = json!({
    "name": "emoji-to-do",
    "user_id": user_id,
  });

  let body = hb.render("index", &data).unwrap();
  Ok(HttpResponse::Ok().body(body))
}
