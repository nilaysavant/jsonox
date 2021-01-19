use actix_web::{middleware::Logger, App, HttpServer};
mod constants;
mod controllers;
mod models;
mod utils;
use controllers::rest;
use utils::{banner::get_banner, cli::setup_cli_get_matches};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = setup_cli_get_matches();
    // Gets a value for bind_addr if supplied by user, or defaults to "0.0.0.0:8080"
    let bind_addr = matches.value_of("bind_addr").unwrap_or("0.0.0.0:8080");
    // Enable quite mode check
    let quiet_mode = matches.is_present("quiet");
    if !quiet_mode {
        std::env::set_var("RUST_LOG", "jsonox=debug,actix_web=info");
        std::env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();
    }

    println!("{}", get_banner());
    println!(
        "{green}Server running at:{nc} {orange}http://{bind_addr}{nc}",
        green = "\x1b[0;32m",
        orange = "\x1b[0;33m",
        nc = "\x1b[0m",
        bind_addr = bind_addr
    );
    println!();

    HttpServer::new(|| {
        App::new()
            // Logger Middleware
            .wrap(Logger::default())
            // Controller Endpoint Services
            .service(rest::list_active_paths)
            .service(rest::post_json_to_path)
            .service(rest::get_json_from_path)
            .service(rest::delete_json_from_path)
    })
    .bind(bind_addr)?
    .run()
    .await
}
