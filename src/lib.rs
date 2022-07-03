#![feature(assert_matches)]
use std::net::TcpListener;

use actix_web::{HttpServer, middleware::Logger, web, App, dev::Server};
use handlers::{hello, webhook};
use models::TeamConfigMap;
use sqlx::{SqlitePool, Sqlite};

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
          .route("/webhook/slack/events", web::post().to(webhook::create_slack_events))
  })
  .listen(listener)?
  .run();

  Ok(server)
}
