use actix_web::{error::ErrorNotFound, web, Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::models::team::Team;

use super::get_current_user;

#[derive(Serialize)]
struct TeamResponse {
    id: i64,
    name: String,
    slack_team_id: String,
}

pub async fn get_team(
    connection: web::Data<SqlitePool>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(connection.as_ref(), &req)
        .await
        .ok_or_else(|| ErrorNotFound("user is not found"))?;

    let team = Team::find(connection.as_ref(), &user.slack_team_id)
        .await
        .map_err(ErrorNotFound)?
        .ok_or_else(|| ErrorNotFound("user is not found"))?;

    let team_response = TeamResponse {
        id: team.id,
        name: team.name,
        slack_team_id: team.slack_team_id,
    };

    Ok(HttpResponse::Ok().json(team_response))
}
