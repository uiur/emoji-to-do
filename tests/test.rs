#![feature(assert_matches)]
use std::net::TcpListener;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

#[actix_rt::test]
async fn health_check_works() {
    let host = spawn_app().await;
    println!("{}", host)

    // let client = reqwest::Client::new();
    // let response = client
    //     .get(format!("{}/health_check", host))
    //     .send()
    //     .await
    //     .expect("failed to execute request");

    // assert!(response.status().is_success());
    // let text = response.text().await.unwrap();
    // println!("{}", text);
}

async fn setup_db() -> SqlitePool {
  let connection = SqlitePoolOptions::new()
    .max_connections(1)
    .connect("sqlite::memory:")
    .await.expect("failed to open database");

  sqlx::migrate!("./migrations")
    .run(&connection)
    .await
    .expect("failed to migrate database");

  connection
}

async fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind");
  let port = listener.local_addr().unwrap().port();

  let connection = setup_db().await;

  let server = emoji_to_do::run(listener, connection).expect("Failed to bind address");
  let _ = actix_rt::spawn(server);

  format!("http://127.0.0.1:{}", port)
}
