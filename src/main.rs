#![feature(assert_matches)]

use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use handlers::{hello, webhook};
use models::TeamConfigMap;

mod github;
mod handlers;
mod models;
mod slack;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        let json_config = web::JsonConfig::default();
        let team_config_map = TeamConfigMap::new();

        App::new()
            .app_data(web::Data::new(team_config_map))
            .app_data(json_config)
            .wrap(Logger::default())
            .service(hello::get_hello)
            .service(webhook::create_slack_events)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
