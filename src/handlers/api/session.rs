use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn delete(session: Session) -> actix_web::Result<impl Responder> {
    session.remove("user_id");
    Ok(HttpResponse::NoContent().finish())
}
