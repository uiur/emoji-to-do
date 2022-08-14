use std::net::TcpListener;

use emoji_to_do::entities;
use sea_orm::{DatabaseConnection, EntityTrait, Set, SqlxSqliteConnector};
use sqlx::sqlite::SqlitePoolOptions;

async fn setup_db() -> DatabaseConnection {
    let connection = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to open connection");

    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .expect("failed to migrate database");

    let connection = SqlxSqliteConnector::from_sqlx_sqlite_pool(connection);

    connection
}

pub async fn spawn_app() -> (String, DatabaseConnection) {
    dotenv::from_filename(".env.test").ok();
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
    let port = listener.local_addr().unwrap().port();

    let connection = setup_db().await;

    let server = emoji_to_do::run(listener, connection.clone()).expect("Failed to bind address");
    let _ = actix_rt::spawn(server);

    (format!("http://127.0.0.1:{}", port), connection)
}

pub fn create_api_client(user_id: i32) -> Result<reqwest::Client, Box<dyn std::error::Error>> {
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

pub async fn create_user(
    connection: &DatabaseConnection,
) -> Result<entities::user::Model, Box<dyn std::error::Error>> {
    let active_model = emoji_to_do::entities::user::ActiveModel {
        slack_team_id: Set("TEAM".to_owned()),
        slack_user_id: Set("User".to_owned()),
        slack_token: Set("TOKEN".to_owned()),
        ..Default::default()
    };

    let result = entities::user::Entity::insert(active_model)
        .exec(connection)
        .await
        .unwrap();

    let user = entities::prelude::User::find_by_id(result.last_insert_id)
        .one(connection)
        .await
        .unwrap()
        .unwrap();

    Ok(user)
}
