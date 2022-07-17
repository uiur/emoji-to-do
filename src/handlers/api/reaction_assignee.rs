use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web, Error, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::models::{
    reaction::{self, Reaction},
    reaction_assignee::{self, ReactionAssignee},
    team::Team,
};

use super::get_current_user;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReactionAssigneeRequestBody {
    pub name: String,
}

pub async fn create_reaction_assignee(
    connection: web::Data<SqlitePool>,
    path: web::Path<(i64,)>,
    req: HttpRequest,
    body: web::Json<CreateReactionAssigneeRequestBody>,
) -> actix_web::Result<impl Responder> {
    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let (reaction_id,) = path.into_inner();
    let reaction = Reaction::find(&connection, reaction_id)
        .await?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    let team = reaction
        .team(&connection)
        .await?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if user.slack_team_id != team.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    let reaction_assignee_id =
        ReactionAssignee::create(&connection, reaction.id, &body.name).await?;

    let reaction_assignee = ReactionAssignee::find(&connection, reaction_assignee_id)
        .await?
        .ok_or_else(|| ErrorNotFound("reaction assignee is not found"))?;

    Ok(HttpResponse::Created().json(reaction_assignee))
}

pub async fn destroy_reaction_assignee(
    connection: web::Data<SqlitePool>,
    path: web::Path<(i64,)>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let (reaction_assignee_id,) = path.into_inner();
    let reaction_assignee = ReactionAssignee::find(&connection, reaction_assignee_id)
        .await?
        .ok_or_else(|| ErrorNotFound("reaction assignee is not found"))?;

    let reaction = Reaction::find(&connection, reaction_assignee.reaction_id)
        .await?
        .ok_or_else(|| ErrorNotFound("reaction is not found"))?;

    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = reaction
        .team(&connection)
        .await?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    if team.slack_team_id != user.slack_team_id {
        return Err(ErrorNotFound("reaction is not found"));
    }

    ReactionAssignee::destroy(&connection, reaction_assignee.id).await?;

    Ok(HttpResponse::NoContent().finish())
}
