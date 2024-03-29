use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

pub async fn get_token(session: Session) -> actix_web::Result<impl Responder> {
    let optional_user_id = session
        .get::<i32>("user_id")
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match optional_user_id {
        Some(user_id) => {
            let token = crate::token::generate(user_id)
                .map_err(actix_web::error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().json(TokenResponse { token }))
        }
        None => Ok(HttpResponse::BadRequest().finish()),
    }
}
