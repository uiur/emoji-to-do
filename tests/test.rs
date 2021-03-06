use std::net::TcpListener;

use emoji_to_do::models::user::User;
use sqlx::{sqlite::SqlitePoolOptions, Sqlite, SqlitePool};

async fn setup_db() -> SqlitePool {
    let connection = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to open database");

    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .expect("failed to migrate database");

    connection
}

pub async fn spawn_app() -> (String, SqlitePool) {
    dotenv::from_filename(".env.test").ok();
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
    let port = listener.local_addr().unwrap().port();

    let connection = setup_db().await;

    let server = emoji_to_do::run(listener, connection.clone()).expect("Failed to bind address");
    let _ = actix_rt::spawn(server);

    (format!("http://127.0.0.1:{}", port), connection)
}

pub fn create_api_client(user_id: i64) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
    let token = emoji_to_do::token::generate(user_id)?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

pub async fn create_user(connection: &SqlitePool) -> Result<User, Box<dyn std::error::Error>> {
    let user_id =
        emoji_to_do::models::user::User::create(connection, "TEAM", "USER", "TOKEN").await?;
    let user = emoji_to_do::models::user::User::find(connection, user_id)
        .await?
        .unwrap();
    Ok(user)
}
