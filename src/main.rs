use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod models;
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
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
