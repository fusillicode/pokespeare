mod errors;
mod fun_translations_client;
mod log_helpers;
mod poke_api_client;
mod services;
mod services_api_models;

use actix_slog::StructuredLogger;
use actix_web::middleware::Compress;
use actix_web::{App, HttpServer};
use log_helpers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log = get_root_logger();
    std::env::set_var("RUST_LOG", "actix_web=info");

    let listen_addr =
        std::env::var("POKESPEARE_LISTEN_ADDR").expect("Missing required POKESPEARE_LISTEN_ADDR");

    info!(log, "Start server"; "listen_addr" => ?listen_addr);
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(StructuredLogger::new(log.clone()))
            .data(log.clone())
            .configure(services::config_app)
    })
    .bind(listen_addr)?
    .run()
    .await
}
