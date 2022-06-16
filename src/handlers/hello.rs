use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
pub async fn get_hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}
