#![feature(assert_matches)]

use actix_web::{Responder, HttpResponse, App, HttpServer, get, post, middleware::Logger, web};
use handlers::{webhook, hello};
use models::TeamConfigMap;

mod handlers;
mod models;
mod slack;
mod github;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
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
