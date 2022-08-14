

mod test;

#[actix_rt::test]
async fn test_slack_auth() {
    let (host, _) = test::spawn_app().await;

    let redirect_policy = reqwest::redirect::Policy::none();
    let client = reqwest::Client::builder()
        .redirect(redirect_policy)
        .build()
        .expect("client build failed");

    let response = client
        .get(format!("{}/auth/slack", host))
        .send()
        .await
        .expect("failed to fetch api");

    assert_eq!(response.status(), 307);

    let body = response.text().await.expect("failed to fetch body");

    println!("{}", body);
}
