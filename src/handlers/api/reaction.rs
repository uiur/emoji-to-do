use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web, HttpRequest, HttpResponse, Responder,
};

use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait, Set};
use serde::{Deserialize, Serialize};

use crate::entities;

use super::get_current_user;

#[derive(Debug, Serialize, Deserialize)]
struct ReactionResponse {
    id: i32,
    name: String,
    repo: String,
    reaction_assignees: Vec<entities::reaction_assignee::Model>,
}

pub async fn get_reactions(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let (team_id,) = path.into_inner();
    let team = entities::prelude::Team::find_by_id(team_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("team is not found"));
    }

    let reactions = team
        .find_related(entities::prelude::Reaction)
        .find_with_related(entities::prelude::ReactionAssignee)
        .all(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    let result: Vec<ReactionResponse> = reactions
        .into_iter()
        .map(|(reaction, reaction_assignees)| ReactionResponse {
            id: reaction.id,
            name: reaction.name.clone(),
            repo: reaction.repo.clone(),
            reaction_assignees,
        })
        .collect();

    Ok(HttpResponse::Ok().json(result))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReactionRequestBody {
    pub name: String,
    pub repo: String,
}

pub async fn create_reaction(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
    body: web::Json<CreateReactionRequestBody>,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let (team_id,) = path.into_inner();
    let team = entities::prelude::Team::find_by_id(team_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if user.slack_team_id != team.slack_team_id {
        return Err(ErrorNotFound("team is not found"));
    }

    let reaction = entities::reaction::ActiveModel {
        team_id: Set(team.id),
        name: Set(body.name.clone()),
        repo: Set(body.repo.clone()),
        ..Default::default()
    }
    .save(connection.as_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    let reaction = entities::prelude::Reaction::find_by_id(reaction.id.unwrap())
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    Ok(HttpResponse::Created().json(reaction))
}

pub async fn get_reaction(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let (reaction_id,) = path.into_inner();
    let reaction = entities::prelude::Reaction::find_by_id(reaction_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("reaction not found"))?;

    let team = reaction
        .find_related(entities::prelude::Team)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team not found"))?;

    if user.slack_team_id != team.slack_team_id {
        return Ok(HttpResponse::Forbidden().finish());
    }

    Ok(HttpResponse::Ok().json(reaction))
}

pub type UpdateReactionRequestBody = CreateReactionRequestBody;

pub async fn put_reaction(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
    body: web::Json<UpdateReactionRequestBody>,
) -> actix_web::Result<impl Responder> {
    let (reaction_id,) = path.into_inner();

    let reaction = entities::prelude::Reaction::find_by_id(reaction_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    let user = get_current_user(connection.as_ref(), &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = entities::prelude::Team::find_by_id(reaction.team_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    let mut active_model = reaction.into_active_model();
    active_model.name = Set(body.name.clone());
    active_model.repo = Set(body.repo.clone());

    active_model
        .save(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    let reaction = entities::prelude::Reaction::find_by_id(reaction_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    Ok(HttpResponse::Ok().json(reaction))
}

pub async fn destroy_reaction(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let (reaction_id,) = path.into_inner();
    let reaction = entities::prelude::Reaction::find_by_id(reaction_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = entities::prelude::Team::find_by_id(reaction.team_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    entities::prelude::Reaction::delete_by_id(reaction.id)
        .exec(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
