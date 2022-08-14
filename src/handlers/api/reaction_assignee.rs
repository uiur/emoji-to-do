use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web, HttpRequest, HttpResponse, Responder,
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde::{Deserialize, Serialize};

use crate::entities;

use super::get_current_user;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReactionAssigneeRequestBody {
    pub name: String,
}

pub async fn create_reaction_assignee(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
    body: web::Json<CreateReactionAssigneeRequestBody>,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let (reaction_id,) = path.into_inner();
    let reaction = entities::prelude::Reaction::find_by_id(reaction_id)
        .one(connection.as_ref())
        .await
        .map_err(|_| ErrorNotFound("reaction is not found"))?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    let team = reaction
        .find_related(entities::prelude::Team)
        .one(connection.as_ref())
        .await
        .map_err(|_| ErrorNotFound("team is not found"))?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if user.slack_team_id != team.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    let reaction_assignee = entities::reaction_assignee::ActiveModel {
        reaction_id: Set(reaction.id),
        name: Set(body.name.clone()),
        ..Default::default()
    }
    .insert(connection.as_ref())
    .await
    .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(reaction_assignee))
}

pub async fn destroy_reaction_assignee(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i32,)>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let (reaction_assignee_id,) = path.into_inner();
    let reaction_assignee = entities::prelude::ReactionAssignee::find_by_id(reaction_assignee_id)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("reaction assignee is not found"))?;

    let reaction = reaction_assignee
        .find_related(entities::prelude::Reaction)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    let user = get_current_user(connection.as_ref(), &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = reaction
        .find_related(entities::prelude::Team)
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    reaction_assignee
        .delete(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
