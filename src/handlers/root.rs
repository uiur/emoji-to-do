use actix_web::{Responder, HttpResponse, web};
use handlebars::Handlebars;
use serde_json::json;

pub async fn get_index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
  let data = json!({
    "name": "emoji-to-do"
  });
  let body = hb.render("index", &data).unwrap();
  HttpResponse::Ok().body(body)
}
