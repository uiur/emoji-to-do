#![feature(assert_matches)]
use std::net::TcpListener;

use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use handlers::{auth, hello, webhook};
use models::TeamConfigMap;
use sqlx::{Sqlite, SqlitePool};

mod github;
mod handlers;
mod models;
mod slack;

pub fn run(listener: TcpListener, connection: SqlitePool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        let json_config = web::JsonConfig::default();
        let team_config_map = TeamConfigMap::new();

        App::new()
            .app_data(web::Data::new(team_config_map))
            .app_data(json_config)
            .app_data(connection.clone())
            .wrap(Logger::default())
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
