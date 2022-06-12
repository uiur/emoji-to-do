use actix_web::{Responder, HttpResponse, App, HttpServer, get, post, middleware::Logger, web};
use handlers::{webhook, hello};


mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        let json_config = web::JsonConfig::default();

        App::new()
            .app_data(json_config)
            .wrap(Logger::default())
            .service(hello::get_hello)
            .service(webhook::create_slack_events)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
