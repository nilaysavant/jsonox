use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
mod constants;
mod controllers;
mod models;
mod utils;
use controllers::rest;
use models::app_state::AppMode;
use models::app_state::AppState;
use utils::{banner::get_banner, cli::setup_cli_get_matches};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // App state variable
    let mut app_state = AppState {
        mode: AppMode::Normal,
    };

    let matches = setup_cli_get_matches();
    // Gets a value for bind_addr if supplied by user, or defaults to "0.0.0.0:8080"
    let bind_addr = matches.value_of("bind_addr").unwrap_or("0.0.0.0:8080");
    // Enable quite mode check
    if !matches.is_present("quiet") {
        std::env::set_var("RUST_LOG", "jsonox=debug,actix_web=info");
        std::env::set_var("RUST_BACKTRACE", "1");
        env_logger::init();
    }
    // Check and set for read only mode
    if matches.is_present("read-only") {
        app_state.mode = AppMode::ReadOnly;
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

    HttpServer::new(move || {
        App::new()
            // Setup App State
            .data(app_state.clone())
            // Logger Middleware
            .wrap(Logger::default())
            // Cors Middleware
            .wrap(Cors::permissive())
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
