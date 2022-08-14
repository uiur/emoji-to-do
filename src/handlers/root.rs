use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use handlebars::Handlebars;
use serde_json::json;
use sqlx::SqlitePool;

pub async fn get_index(
    connection: web::Data<sea_orm::DatabaseConnection>,
    hb: web::Data<Handlebars<'_>>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    let data = json!({
      "name": "emoji-to-do",
    });

    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
