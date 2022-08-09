use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web, Error, HttpRequest, HttpResponse, Responder,
};
use futures::future::try_join_all;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::models::{reaction::Reaction, reaction_assignee::ReactionAssignee, team::Team};

use super::get_current_user;

#[derive(Debug, Serialize, Deserialize)]
struct ReactionResponse {
    id: i64,
    name: String,
    repo: String,
    reaction_assignees: Vec<ReactionAssignee>,
}

pub async fn get_reactions(
    connection: web::Data<sea_orm::DatabaseConnection>,
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

    let reaction_assignees_results = try_join_all(
        reactions
            .iter()
            .map(|reaction| ReactionAssignee::search_by_reaction_id(&connection, reaction.id)),
    )
    .await?;

    let result: Vec<ReactionResponse> = reactions
        .iter()
        .zip(reaction_assignees_results)
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

pub type UpdateReactionRequestBody = CreateReactionRequestBody;

pub async fn put_reaction(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64,)>,
    req: HttpRequest,
    body: web::Json<UpdateReactionRequestBody>,
) -> actix_web::Result<impl Responder> {
    let (reaction_id,) = path.into_inner();
    let reaction = Reaction::find(&connection, reaction_id)
        .await?
        .ok_or(ErrorNotFound("reaction is not found"))?;

    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = Team::find_by_id(&connection, reaction.team_id)
        .await?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    Reaction::update(&connection, reaction.id, &body.name, &body.repo).await?;
    let reaction = Reaction::find(&connection, reaction_id)
        .await?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    Ok(HttpResponse::Ok().json(reaction))
}

pub async fn destroy_reaction(
    connection: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64,)>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let (reaction_id,) = path.into_inner();
    let reaction = Reaction::find(&connection, reaction_id)
        .await?
        .ok_or(ErrorNotFound("reaction is not found"))?;

    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = Team::find_by_id(&connection, reaction.team_id)
        .await?
        .ok_or(ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    Reaction::destroy(&connection, reaction.id).await?;

    Ok(HttpResponse::NoContent().finish())
}
