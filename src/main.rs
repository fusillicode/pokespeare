mod log_helpers;
mod poke_api_client;

use actix_slog::StructuredLogger;
use actix_web::middleware::Compress;
use actix_web::web::{Data, Path};
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use log_helpers::*;
use poke_api_client::PokeApiClient;
use reqwest::Url;
use serde::{Deserialize, Serialize};

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
    let fun_translations_client = FunTranslationsClient {
        endpoint: fun_translations_api_endpoint,
    };

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

#[derive(Clone)]
struct FunTranslationsClient {
    endpoint: Url,
}

#[get("/pokemon/{pokemon_id_or_name}")]
async fn get_shakesperean_description(
    log: Data<Logger>,
    poke_api_client: Data<PokeApiClient>,
    fun_translations_client: Data<FunTranslationsClient>,
    pokemon_id_or_name: Path<String>,
) -> Result<HttpResponse, Error> {
    let mut fun_translations_shakespere_request_url = fun_translations_client.endpoint.clone();
    fun_translations_shakespere_request_url
        .path_segments_mut()
        .unwrap()
        .push("shakespeare.json");

    let cleaned_random_en_description = poke_api_client
        .get_random_description(&pokemon_id_or_name)
        .await
        .unwrap();

    info!(log, "PokeApi data"; "pokemon_description" => ?cleaned_random_en_description);

    let shakesperean_description_response = reqwest::Client::new()
        .get(fun_translations_shakespere_request_url)
        .query(&[("text", &cleaned_random_en_description)])
        .send()
        .await;

    info!(log, "FunTranslations response"; "response" => ?shakesperean_description_response);

    let shakesperean_description = shakesperean_description_response
        .unwrap()
        .json::<ShakespereanDescription>()
        .await
        .unwrap();

    Ok(HttpResponse::Ok().json(shakesperean_description.contents.translated_text))
}

#[derive(Deserialize, Serialize)]
struct ShakespereanDescription {
    contents: ShakespereanDescriptionContents,
}

#[derive(Deserialize, Serialize)]
struct ShakespereanDescriptionContents {
    #[serde(rename = "translated")]
    translated_text: String,
    #[serde(rename = "text")]
    original_text: String,
}
