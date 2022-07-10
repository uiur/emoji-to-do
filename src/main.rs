#![feature(assert_matches)]
use std::net::TcpListener;

use emoji_to_do::run;

use sqlx::sqlite::SqlitePoolOptions;

mod github;
mod handlers;
mod models;
mod slack;
mod token;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("failed to open database");

    let listener = TcpListener::bind("127.0.0.1:8080").expect("failed to bind");
    run(listener, pool)?.await
}
