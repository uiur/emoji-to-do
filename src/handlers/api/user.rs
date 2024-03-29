use actix_web::{web, HttpRequest, HttpResponse, Responder};

use super::get_current_user;

pub async fn get_user(
    connection: web::Data<sea_orm::DatabaseConnection>,
    req: HttpRequest,
) -> impl Responder {
    let optional_user = get_current_user(connection.as_ref(), &req).await;

    match optional_user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}
