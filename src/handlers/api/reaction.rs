use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web, Error, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::models::{reaction::Reaction, team::Team};

use super::get_current_user;

pub async fn get_reactions(
    connection: web::Data<SqlitePool>,
    path: web::Path<(i64,)>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let (team_id,) = path.into_inner();
    let team = Team::find_by_id(&connection, team_id)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("team is not found"));
    }

    let reactions = team
        .reactions(&connection)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(reactions))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReactionRequestBody {
    pub name: String,
    pub repo: String,
}

pub async fn create_reaction(
    connection: web::Data<SqlitePool>,
    path: web::Path<(i64,)>,
    req: HttpRequest,
    body: web::Json<CreateReactionRequestBody>,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let (team_id,) = path.into_inner();
    let team = Team::find_by_id(&connection, team_id)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if user.slack_team_id != team.slack_team_id {
        return Err(ErrorNotFound("team is not found"));
    }

    let reaction_id = Reaction::create(&connection, team.id, &body.name, &body.repo)
        .await
        .map_err(ErrorInternalServerError)?;

    let reaction = Reaction::find(&connection, reaction_id)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorInternalServerError(""))?;
    println!("{:?}", body);

    Ok(HttpResponse::Created().json(reaction))
}
