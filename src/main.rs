#![feature(assert_matches)]
use std::{env, net::TcpListener};

use emoji_to_do::run;

use listenfd::ListenFd;
use sea_orm::Database;

mod entities;
mod github;
mod handlers;
mod slack;
mod token;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let connection = Database::connect(&database_url)
        .await
        .expect("failed to open database");

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let mut listenfd = ListenFd::from_env();

    let listener = match listenfd.take_tcp_listener(0)? {
        Some(listener) => listener,
        None => TcpListener::bind(server_url).expect("failed to bind"),
    };
    run(listener, connection)?.await
}
