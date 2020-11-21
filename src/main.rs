mod log_helpers;

use actix_slog::StructuredLogger;
use actix_web::middleware::Compress;
use actix_web::web::{Data, Path};
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use log_helpers::*;
use reqwest::Url;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log = get_root_logger();
    std::env::set_var("RUST_LOG", "actix_web=info");

    let listen_addr =
        std::env::var("POKESPEARE_LISTEN_ADDR").expect("Missing required POKESPEARE_LISTEN_ADDR");
    let poke_api_endpoint = Url::parse(
        &std::env::var("POKE_API_ENDPOINT").expect("Missing required POKE_API_ENDPOINT"),
    )
    .unwrap();
    let fun_translations_api_endpoint = Url::parse(
        &std::env::var("FUN_TRANSLATIONS_API_ENDPOINT")
            .expect("Missing required FUN_TRANSLATIONS_API_ENDPOINT"),
    )
    .unwrap();

    let poke_api_client = PokeApiClient {
        endpoint: poke_api_endpoint,
    };
    let fun_translations_api_client = FunTranslationsApiClient {
        endpoint: fun_translations_api_endpoint,
    };

    info!(log, "Start server"; "listen_addr" => ?listen_addr);
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(StructuredLogger::new(log.clone()))
            .data(poke_api_client.clone())
            .data(fun_translations_api_client.clone())
            .service(get_shakesperean_description)
    })
    .bind(listen_addr)?
    .run()
    .await
}

#[get("/pokemon/{pokemon_id_or_name}")]
async fn get_shakesperean_description(
    poke_api_client: Data<PokeApiClient>,
    fun_translations_api_client: Data<FunTranslationsApiClient>,
    pokemon_id_or_name: Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[derive(Clone)]
struct PokeApiClient {
    endpoint: Url,
}

#[derive(Clone)]
struct FunTranslationsApiClient {
    endpoint: Url,
}
