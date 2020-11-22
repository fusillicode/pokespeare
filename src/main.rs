mod fun_translations_client;
mod log_helpers;
mod poke_api_client;

use actix_slog::StructuredLogger;
use actix_web::middleware::Compress;
use actix_web::web::{Data, Path};
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use fun_translations_client::FunTranslationsClient;
use log_helpers::*;
use poke_api_client::PokeApiClient;
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log = get_root_logger();
    std::env::set_var("RUST_LOG", "actix_web=info");

    let listen_addr =
        std::env::var("POKESPEARE_LISTEN_ADDR").expect("Missing required POKESPEARE_LISTEN_ADDR");
    let poke_api_endpoint =
        std::env::var("POKE_API_ENDPOINT").expect("Missing required POKE_API_ENDPOINT");
    let fun_translations_api_endpoint = std::env::var("FUN_TRANSLATIONS_API_ENDPOINT")
        .expect("Missing required FUN_TRANSLATIONS_API_ENDPOINT");

    let poke_api_client = PokeApiClient::new(&poke_api_endpoint);
    let fun_translations_client = FunTranslationsClient::new(&fun_translations_api_endpoint);

    info!(log, "Start server"; "listen_addr" => ?listen_addr);
    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(StructuredLogger::new(log.clone()))
            .data(log.clone())
            .data(poke_api_client.clone())
            .data(fun_translations_client.clone())
            .service(get_shakesperean_description)
    })
    .bind(listen_addr)?
    .run()
    .await
}

#[get("/pokemon/{pokemon_name}")]
async fn get_shakesperean_description(
    _log: Data<Logger>,
    poke_api_client: Data<PokeApiClient>,
    fun_translations_client: Data<FunTranslationsClient>,
    pokemon_name: Path<String>,
) -> Result<HttpResponse, Error> {
    let pokemon_description = poke_api_client
        .get_random_description(&pokemon_name)
        .await
        .unwrap();

    let shakesperean_description = fun_translations_client
        .translate(&pokemon_description)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(ShakespereanDescriptionApiResponse {
        name: pokemon_name.to_string(),
        description: shakesperean_description,
    }))
}

#[derive(Serialize)]
struct ShakespereanDescriptionApiResponse {
    name: String,
    description: String,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_everything_is_fine() {
        assert!(true);
    }

    // #[test]
    // fn test_poke_api_returns_status_code_different_from_200() {
    //     assert!(true);
    // }

    // #[test]
    // fn test_poke_apis_returns_200_with_unexpected_body() {
    //     assert_eq!(true, true);
    // }

    // #[test]
    // fn test_poke_apis_returns_200_without_a_traslatable_description() {
    //     assert_eq!(true, true);
    // }

    // #[test]
    // fn test_fun_translations_returns_status_code_different_from_200() {
    //     assert_eq!(true, true);
    // }

    // #[test]
    // fn test_fun_translations_returns_200_with_unexpected_body() {
    //     assert_eq!(true, true);
    // }
}
