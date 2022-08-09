use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

pub async fn get_hello(connection: web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    let _result = sqlx::query!("select 1 as one;")
        .fetch_all(connection.as_ref())
        .await;

    HttpResponse::Ok().body("hello")
}
