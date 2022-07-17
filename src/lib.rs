#![feature(assert_matches)]
use std::{env, net::TcpListener};

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, dev::Server, middleware::Logger, web, App, HttpServer};
use handlebars::Handlebars;
use handlers::{api, auth, hello, root, webhook};
use sqlx::SqlitePool;

mod github;
mod handlers;
pub mod models;
mod slack;
pub mod token;

pub fn run(listener: TcpListener, connection: SqlitePool) -> Result<Server, std::io::Error> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let master_key = env::var("MASTER_KEY").expect("MASTER_KEY is expected");
    let secret_key = Key::derive_from(master_key.as_bytes());

    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        let json_config = web::JsonConfig::default();

        App::new()
            .app_data(json_config)
            .app_data(connection.clone())
            .app_data(handlebars_ref.clone())
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .route("/", web::get().to(root::get_index))
            .route("/hello", web::get().to(hello::get_hello))
            .route("/auth/slack", web::get().to(auth::get_slack_auth))
            .route(
                "/auth/slack/callback",
                web::get().to(auth::slack_auth_callback),
            )
            .route(
                "/webhook/slack/events",
                web::post().to(webhook::create_slack_events),
            )
            .route("/api/user", web::get().to(api::user::get_user))
            .route("/api/token", web::get().to(api::token::get_token))
            .route("/api/team", web::get().to(api::team::get_team))
            .route(
                "/api/teams/{team_id}/reactions",
                web::get().to(api::reaction::get_reactions),
            )
            .route(
                "/api/teams/{team_id}/reactions",
                web::post().to(api::reaction::create_reaction),
            )
            .route(
                "/api/reactions/{reaction_id}",
                web::delete().to(api::reaction::destroy_reaction),
            )
            .route(
                "/api/reactions/{reaction_id}",
                web::put().to(api::reaction::put_reaction),
            )
            .route(
                "/api/reactions/{reaction_id}/reaction_assignees",
                web::post().to(api::reaction_assignee::create_reaction_assignee),
            )
            .route(
                "/api/reaction_assignees/{reaction_assignee_id}",
                web::delete().to(api::reaction_assignee::destroy_reaction_assignee),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
