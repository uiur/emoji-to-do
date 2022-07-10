use test::spawn_app;
mod test;

#[actix_rt::test]
async fn test_hello_returns_success() {
    let (host, _) = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/hello", host))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    let text = response.text().await.unwrap();
    println!("{}", text);
}
