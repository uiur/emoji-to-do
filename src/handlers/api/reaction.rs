use std::{collections::HashSet, option};

use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web, HttpRequest, HttpResponse, Responder,
};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

use crate::entities::{self, reaction_assignee};

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
    pub reaction_assignees: Vec<CreateReactionRequestReactionAssignee>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReactionRequestReactionAssignee {
    pub name: String,
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
    let reaction_id = reaction.id.unwrap();

    for body in &body.reaction_assignees {
        entities::reaction_assignee::ActiveModel {
            reaction_id: Set(reaction_id),
            name: Set(body.name.clone()),
            ..Default::default()
        }
        .insert(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;
    }

    let reaction = entities::prelude::Reaction::find_by_id(reaction_id)
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

    let mut reaction_assignee_ids: HashSet<i32> = HashSet::new();
    for reaction_assignee_body in &body.reaction_assignees {
        let optional_reaction_assignee = reaction
            .find_related(entities::prelude::ReactionAssignee)
            .filter(
                entities::reaction_assignee::Column::Name.eq(reaction_assignee_body.name.clone()),
            )
            .one(connection.as_ref())
            .await
            .map_err(ErrorInternalServerError)?;

        match optional_reaction_assignee {
            Some(reaction_assignee) => {
                reaction_assignee_ids.insert(reaction_assignee.id);
            }
            None => {
                let reaction_assignee = entities::reaction_assignee::ActiveModel {
                    reaction_id: Set(reaction.id),
                    name: Set(reaction_assignee_body.name.clone()),
                    ..Default::default()
                }
                .insert(connection.as_ref())
                .await
                .map_err(ErrorInternalServerError)?;
                reaction_assignee_ids.insert(reaction_assignee.id);
            }
        }
    }

    let reaction_assignees = reaction
        .find_related(entities::prelude::ReactionAssignee)
        .all(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    let ids = HashSet::from_iter(
        reaction_assignees
            .iter()
            .map(|reaction_assignee| reaction_assignee.id),
    );
    let ids_to_remove: Vec<_> = ids.difference(&reaction_assignee_ids).cloned().collect();
    entities::prelude::ReactionAssignee::delete_many()
        .filter(entities::reaction_assignee::Column::Id.is_in(ids_to_remove))
        .exec(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

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
