mod test;

#[actix_rt::test]
async fn slack_auth() {
    let host = test::spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/auth/slack", host))
        .send()
        .await
        .expect("failed to fetch api");

    assert!(response.status().is_success());

    let body = response.text().await.expect("failed to fetch body");

    println!("{}", body);
}
