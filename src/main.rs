use actix_web::{get, middleware::Logger, post, App, HttpResponse, HttpServer, Responder};
mod constants;
mod controllers;
mod models;
mod utils;
use controllers::rest;
use models::server_error::{map_to_server_error, ServerError};

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
            .service(rest::post_json_to_path)
            .service(rest::get_json_from_path)
            .service(rest::delete_json_from_path)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
