#![feature(assert_matches)]
use std::{net::TcpListener, env};

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer, cookie::Key};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use handlebars::Handlebars;
use handlers::{auth, hello, webhook, root};
use models::TeamConfigMap;
use sqlx::{Sqlite, SqlitePool};

mod github;
mod handlers;
mod models;
mod slack;

pub fn run(listener: TcpListener, connection: SqlitePool) -> Result<Server, std::io::Error> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let master_key = env::var("MASTER_KEY").unwrap();
    let secret_key = Key::derive_from(master_key.as_bytes());

    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        let json_config = web::JsonConfig::default();
        let team_config_map = TeamConfigMap::new();

        App::new()
            .app_data(web::Data::new(team_config_map))
            .app_data(json_config)
            .app_data(connection.clone())
            .app_data(handlebars_ref.clone())
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            .route("/", web::get().to(root::get_index))
            .route("/hello", web::get().to(hello::get_hello))
            .route("/auth/slack", web::get().to(auth::get_slack_auth))
            .route("/auth/slack/callback", web::get().to(auth::slack_auth_callback))
            .route(
                "/webhook/slack/events",
                web::post().to(webhook::create_slack_events),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
