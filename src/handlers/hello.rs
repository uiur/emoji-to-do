use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

pub async fn get_hello(connection: web::Data<sea_orm::DatabaseConnection>) -> impl Responder {
    connection
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "select 1 as one;".to_owned(),
        ))
        .await
        .expect("query failed");

    HttpResponse::Ok().body("hello")
}
