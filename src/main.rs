mod fun_translations_client;
mod log_helpers;
mod poke_api_client;

use actix_slog::StructuredLogger;
use actix_web::middleware::Compress;
use actix_web::web::{Data, Path, ServiceConfig};
use actix_web::{get, App, Error, HttpResponse, HttpServer};
use fun_translations_client::FunTranslationsClient;
use log_helpers::*;
use poke_api_client::PokeApiClient;
use serde::{Deserialize, Serialize};

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
            .configure(config_app)
    })
    .bind(listen_addr)?
    .run()
    .await
}

fn config_app(cfg: &mut ServiceConfig) {
    let poke_api_endpoint =
        std::env::var("POKE_API_ENDPOINT").expect("Missing required POKE_API_ENDPOINT");
    let fun_translations_api_endpoint = std::env::var("FUN_TRANSLATIONS_API_ENDPOINT")
        .expect("Missing required FUN_TRANSLATIONS_API_ENDPOINT");

    let poke_api_client = PokeApiClient::new(&poke_api_endpoint);
    let fun_translations_client = FunTranslationsClient::new(&fun_translations_api_endpoint);

    cfg.data(poke_api_client);
    cfg.data(fun_translations_client);
    cfg.service(get_shakesperean_description);
}

#[get("/pokemon/{pokemon_name}")]
async fn get_shakesperean_description(
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ShakespereanDescriptionApiResponse {
    name: String,
    description: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{dev::ServiceResponse, test, test::TestRequest};
    use mockito::{mock, Matcher};

    fn set_up_mocks() -> (PokeApiClient, FunTranslationsClient) {
        let mock_server_url = mockito::server_url();

        std::env::set_var("POKE_API_ENDPOINT", &mock_server_url);
        std::env::set_var("FUN_TRANSLATIONS_API_ENDPOINT", &mock_server_url);

        (
            PokeApiClient::new(&mock_server_url),
            FunTranslationsClient::new(&mock_server_url),
        )
    }

    async fn call_get_shakesperean_description_service(pokemon_name: &str) -> ServiceResponse {
        let (poke_api_client, fun_translations_client) = set_up_mocks();
        let mut app = test::init_service(App::new().configure(config_app)).await;
        let req = TestRequest::get()
            .uri(&format!("/pokemon/{}", pokemon_name))
            .data(poke_api_client)
            .data(fun_translations_client)
            .to_request();
        test::call_service(&mut app, req).await
    }

    #[actix_rt::test]
    async fn test_everything_is_fine() {
        let pokemon_name = "bulbasaur";

        let _poke_api_mock = mock("GET", format!("/api/v2/pokemon-species/{}", pokemon_name).as_str())
            .with_status(200)
            .with_body(
                std::fs::read_to_string("./tests/fixtures/poke_api_ok_response.json").unwrap(),
            )
            .create();
        let _fun_translations_mock = mock("GET", "/translate/shakespeare.json")
            .match_query(Matcher::Regex("text=.*".into()))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("./tests/fixtures/fun_translations_ok_response.json")
                    .unwrap(),
            )
            .create();

        let resp = call_get_shakesperean_description_service(pokemon_name).await;

        assert!(resp.status().is_success());
        assert_eq!(
            ShakespereanDescriptionApiResponse {
                name: pokemon_name.into(),
                description: "A strange seed wast planted on its back at birth. The plant sprouts and grows with this pokémon.".into(),
            },
            test::read_body_json(resp).await
        );
    }

    // #[actix_rt::test]
    // async fn test_poke_api_returns_status_code_different_from_200() {
    //     let pokemon_name = "bulbasaur";

    //     let _poke_api_mock = mock("GET", format!("/api/v2/pokemon-species/{}", pokemon_name).as_str())
    //         .with_status(404)
    //         .create();

    //     let resp = call_get_shakesperean_description_service(pokemon_name).await;

    //     assert!(resp.status().is_server_error());
    // }

    // #[actix_rt::test]
    // async fn test_poke_apis_returns_200_with_unexpected_body() {
    //     let pokemon_name = "bulbasaur";

    //     let _poke_api_mock = mock("GET", format!("/api/v2/pokemon-species/{}", pokemon_name).as_str())
    //         .with_status(200)
    //         .with_body("That's the body you're looking for...")
    //         .create();

    //     let resp = call_get_shakesperean_description_service(pokemon_name).await;

    //     assert!(resp.status().is_server_error());
    // }

    // #[actix_rt::test]
    // async fn test_poke_apis_returns_200_without_a_traslatable_description() {
    //     let pokemon_name = "bulbasaur";

    //     let _poke_api_mock = mock("GET", format!("/api/v2/pokemon-species/{}", pokemon_name).as_str())
    //     .with_status(200)
    //     .with_body(
    //         std::fs::read_to_string("./tests/fixtures/poke_api_not_translatable_description_response.json").unwrap(),
    //     )
    //     .create();

    //     let resp = call_get_shakesperean_description_service(pokemon_name).await;

    //     assert!(resp.status().is_server_error());
    // }

    // #[actix_rt::test]
    // async fn test_fun_translations_returns_status_code_different_from_200() {
    //     let pokemon_name = "bulbasaur";

    //     let _poke_api_mock = mock("GET", format!("/api/v2/pokemon-species/{}", pokemon_name).as_str())
    //         .with_status(200)
    //         .with_body(
    //             std::fs::read_to_string("./tests/fixtures/poke_api_ok_response.json").unwrap(),
    //         )
    //         .create();

    //     let _fun_translations_mock = mock("GET", "/translate/shakespeare.json")
    //     .match_query(Matcher::Regex("text=.*".into()))
    //     .with_status(429)
    //     .with_body(
    //         std::fs::read_to_string("./tests/fixtures/fun_translations_ok_response.json")
    //             .unwrap(),
    //     )
    //     .create();

    //     let resp = call_get_shakesperean_description_service(pokemon_name).await;

    //     assert!(resp.status().is_server_error());
    // }

    // #[actix_rt::test]
    // async fn test_fun_translations_returns_200_with_unexpected_body() {
    //     let pokemon_name = "bulbasaur";

    //     let _poke_api_mock = mock("GET", format!("/api/v2/pokemon-species/{}", pokemon_name).as_str())
    //         .with_status(200)
    //         .with_body(
    //             std::fs::read_to_string("./tests/fixtures/poke_api_ok_response.json").unwrap(),
    //         )
    //         .create();

    //     let _fun_translations_mock = mock("GET", "/translate/shakespeare.json")
    //     .match_query(Matcher::Regex("text=.*".into()))
    //     .with_status(429)
    //     .with_body("That's the body you're looking for...")
    //     .create();

    //     let resp = call_get_shakesperean_description_service(pokemon_name).await;

    //     assert!(resp.status().is_server_error());
    // }
}
