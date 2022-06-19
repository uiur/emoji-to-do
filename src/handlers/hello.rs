use actix_web::{get, HttpResponse, Responder, web};
use sqlx::{Pool, SqlitePool};

#[get("/hello")]
pub async fn get_hello(connection: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query!("select 1 as one;")
        .fetch_all(connection.as_ref())
        .await;

    HttpResponse::Ok().body("hello")
}
