use std::env;

use actix_session::Session;
use actix_web::{
    error::ErrorInternalServerError,
    web::{self, Query},
    HttpResponse, Responder,
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::entities;

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
struct ExtraFields {
    team: TeamFields,
    authed_user: AuthedUserFields,
}

impl oauth2::ExtraTokenFields for ExtraFields {}

#[derive(Deserialize, Debug, Serialize)]
struct TeamFields {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct AuthedUserFields {
    id: String,
}

fn create_oauth_client() -> OauthClient {
    let client_id = env::var("SLACK_CLIENT_ID").expect("SLACK_CLIENT_ID is expected");
    let client_secret = env::var("SLACK_CLIENT_SECRET").expect("SLACK_CLIENT_SECRET is expected");
    let http_host = env::var("E2D_HTTP_HOST").expect("E2D_HTTP_HOST is expected");

    OauthClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://slack.com/oauth/v2/authorize".to_string()).unwrap(),
        Some(TokenUrl::new("https://slack.com/api/oauth.v2.access".to_string()).unwrap()),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new(format!("{}/auth/slack/callback", &http_host)).unwrap())
}

#[derive(Debug, Serialize, Deserialize)]
struct GetSlackAuthResponse {
    url: String,
}

pub async fn get_slack_auth() -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();

    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("users.profile:read".to_string()))
        .url();

    Ok(HttpResponse::Ok().json(GetSlackAuthResponse {
        url: auth_url.to_string(),
    }))
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackAuthCallbackResponse {
    token: String,
}

pub async fn slack_auth_callback(
    connection: web::Data<sea_orm::DatabaseConnection>,
    query: Query<CallbackQuery>,
    session: Session,
) -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();
    log::info!("{}", &query.code);

    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("{:?}", token_result.extra_fields());
    let extra_fields = token_result.extra_fields();
    let token = token_result.access_token();

    let slack_team_id = extra_fields.team.id.clone();
    let slack_user_id = extra_fields.authed_user.id.clone();

    let option_user = entities::prelude::User::find()
        .filter(entities::user::Column::SlackUserId.eq(slack_user_id.clone()))
        .one(connection.as_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    match option_user {
        Some(found_user) => {
            session.insert("user_id", found_user.id);
            let token = crate::token::generate(found_user.id).map_err(ErrorInternalServerError)?;
            Ok(HttpResponse::Ok().json(SlackAuthCallbackResponse { token }))
        }
        None => {
            let active_model = entities::user::ActiveModel {
                slack_team_id: Set(slack_team_id),
                slack_user_id: Set(slack_user_id),
                slack_token: Set(token.secret().clone()),
                ..Default::default()
            };
            let res = entities::user::Entity::insert(active_model)
                .exec(connection.as_ref())
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            session.insert("user_id", res.last_insert_id);

            let token =
                crate::token::generate(res.last_insert_id).map_err(ErrorInternalServerError)?;
            Ok(HttpResponse::Ok().json(SlackAuthCallbackResponse { token }))
        }
    }
}
