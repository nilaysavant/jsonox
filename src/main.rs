use actix_web::{get, middleware::Logger, post, App, HttpResponse, HttpServer, Responder};
mod constants;
mod controllers;
mod models;
mod utils;
use controllers::rest::post_json_to_path;
use models::server_error::{map_to_server_error, ServerError};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> Result<HttpResponse, ServerError> {
    let json_body: serde_json::Value =
        serde_json::from_str(req_body.as_str()).map_err(map_to_server_error)?;
    Ok(HttpResponse::Ok().json(json_body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "jsonox=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // Logger Middleware
            .wrap(Logger::default())
            // Controller Endpoint Services
            .service(post_json_to_path)
            .service(hello)
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
