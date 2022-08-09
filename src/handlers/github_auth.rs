use std::env;

use actix_session::Session;
use actix_web::{
    error::{ErrorInternalServerError, ErrorUnauthorized},
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    github,
    handlers::api::get_current_user,
    models::{team::Team, user::User},
};

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

pub async fn get_github_auth() -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();

    let (auth_url, _csrf_token) = client.authorize_url(CsrfToken::new_random).url();

    Ok(HttpResponse::TemporaryRedirect()
        .insert_header(("Location", auth_url.to_string()))
        .finish())
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
}

pub async fn github_auth_callback(
    connection: web::Data<sea_orm::DatabaseConnection>,
    query: Query<CallbackQuery>,
    session: Session,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();
    let connection = connection.get_database_backend();
    log::info!("{}", &query.code);

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

    let user = get_current_user(&connection, &req)
        .await
        .ok_or_else(|| ErrorUnauthorized(""))?;

    let team = Team::find(&connection, &user.slack_team_id)
        .await
        .map_err(ErrorInternalServerError)?
        .ok_or_else(|| ErrorInternalServerError(""))?;

    team.update(&connection, &team.name, installation.id)
        .await?;

    Ok(HttpResponse::TemporaryRedirect()
        .insert_header(("Location", "/".to_string()))
        .finish())
}
