use std::env;

use actix_session::Session;
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized},
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use log::debug;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse,
    TokenUrl,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::{entities, github, handlers::api::get_current_user};

type OauthClient = oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<ExtraFields, oauth2::basic::BasicTokenType>,
    oauth2::basic::BasicTokenType,
    oauth2::StandardTokenIntrospectionResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
>;

#[derive(Deserialize, Debug, Serialize)]
struct ExtraFields {}

impl oauth2::ExtraTokenFields for ExtraFields {}

#[derive(Deserialize, Debug, Serialize)]
struct AuthedUserFields {
    id: String,
}

fn create_oauth_client() -> OauthClient {
    let client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID is expected");
    let client_secret = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET is expected");
    let http_host = env::var("E2D_HTTP_HOST").expect("E2D_HTTP_HOST is expected");

    OauthClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap()),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(format!("{}/auth/github/callback", &http_host)).unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
struct GetGithubAuthResponse {
    url: String,
}

pub async fn get_github_auth() -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();

    let (auth_url, _csrf_token) = client.authorize_url(CsrfToken::new_random).url();

    Ok(HttpResponse::Ok().json(GetGithubAuthResponse {
        url: auth_url.to_string(),
    }))
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

pub async fn github_auth_callback(
    connection: web::Data<sea_orm::DatabaseConnection>,
    query: Query<CallbackQuery>,
    _session: Session,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();

    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(ErrorInternalServerError)?;

    let extra_fields = token_result.extra_fields();
    let token = token_result.access_token();
    println!("{:?} {:?}", token, extra_fields);

    // let slack_team_id = extra_fields.team.id.clone();
    // let slack_user_id = extra_fields.authed_user.id.clone();

    // let option_user = User::find_by_slack_user_id(&connection, &slack_user_id)
    //     .await
    //     .map_err(actix_web::error::ErrorInternalServerError)?;

    // match option_user {
    //     Some(found_user) => {
    //         session.insert("user_id", found_user.id);
    //     }
    //     None => {
    //         let user_id = User::create(&connection, &slack_team_id, &slack_user_id, token.secret())
    //             .await
    //             .map_err(actix_web::error::ErrorInternalServerError)?;

    //         session.insert("user_id", user_id);
    //     }
    // }
    let installations = github::list_user_installations(token.secret()).await?;

    let installation = installations
        .first()
        .ok_or_else(|| ErrorInternalServerError("installation is not found"))?;

    let user = get_current_user(connection.as_ref(), &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = entities::prelude::Team::find()
        .filter(entities::team::Column::SlackTeamId.eq(user.slack_team_id))
        .one(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorNotFound("team is not found"))?;

    let mut active_model = team.into_active_model();
    active_model.github_installation_id = Set(Some(installation.id));
    active_model
        .save(connection.as_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok())
}
