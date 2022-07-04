use std::env;

use actix_web::{get, web::{self, Query}, HttpResponse, Responder};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, AuthorizationCode, TokenResponse,
};
use serde::Deserialize;
use sqlx::{Pool, SqlitePool};


fn create_oauth_client() -> oauth2::Client<oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>, oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, oauth2::basic::BasicTokenType, oauth2::StandardTokenIntrospectionResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, oauth2::StandardRevocableToken, oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>> {
    let client_id = env::var("SLACK_CLIENT_ID").unwrap();
    let client_secret = env::var("SLACK_CLIENT_SECRET").unwrap();
    let http_host = env::var("E2D_HTTP_HOST").unwrap();

    let client = BasicClient::new(
        ClientId::new(client_id.clone().to_string()),
        Some(ClientSecret::new(client_secret.clone().to_string())),
        AuthUrl::new("https://slack.com/oauth/v2/authorize".to_string()).unwrap(),
        Some(
            TokenUrl::new("https://slack.com/api/oauth.v2.access".to_string()).unwrap(),
        ),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(
        RedirectUrl::new(format!("{}/auth/slack/callback", &http_host)).unwrap()
    );
    client
}

pub async fn get_slack_auth() -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("users.profile:read".to_string()))
        .url();

    Ok(HttpResponse::TemporaryRedirect().insert_header(("Location", auth_url.to_string())).finish())
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String
}

pub async fn slack_auth_callback(query: Query<CallbackQuery>) -> actix_web::Result<impl Responder> {
    let client = create_oauth_client();
    log::info!("{}", &query.code);

    let token_result =
        client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(oauth2::reqwest::async_http_client).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;


    let token = token_result.access_token();
    Ok(HttpResponse::Ok().body(token.secret().to_string()))
}
