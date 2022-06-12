use actix_web::{Responder, HttpResponse, get};

#[get("/hello")]
pub async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
