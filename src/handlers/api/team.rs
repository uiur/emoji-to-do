use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    web, HttpRequest, HttpResponse, Responder,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::entities;

use super::get_current_user;

#[derive(Serialize)]
struct TeamResponse {
    id: i32,
    name: String,
    slack_team_id: String,
}

pub async fn get_team(
    connection: web::Data<sea_orm::DatabaseConnection>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(connection.as_ref(), &req)
        .await
        .ok_or_else(|| ErrorNotFound("user is not found"))?;

    let team = entities::prelude::Team::find()
        .filter(entities::team::Column::SlackTeamId.eq(user.slack_team_id.as_str()))
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    let team_response = TeamResponse {
        id: team.id,
        name: team.name,
        slack_team_id: team.slack_team_id,
    };

    Ok(HttpResponse::Ok().json(team_response))
}
