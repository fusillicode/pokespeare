use actix_web::App;
use actix_web::{dev::ServiceResponse, test, test::TestRequest};
use mockito::{mock, Matcher};
use pokespeare::errors::JsonErrorResponseBody;
use pokespeare::fun_translations_client::FunTranslationsClient;
use pokespeare::poke_api_client::PokeApiClient;
use pokespeare::services;
use pokespeare::services_api_models::ShakespeareanDescriptionApiResponse;

#[actix_rt::test]
async fn test_everything_is_fine() {
    let pokemon_name = "bulbasaur";

    let _poke_api_mock = mock(
        "GET",
        format!("/api/v2/pokemon-species/{}", pokemon_name).as_str(),
    )
    .with_status(200)
    .with_body(std::fs::read_to_string("./tests/fixtures/poke_api_ok_response.json").unwrap())
    .create();
    let _fun_translations_mock = mock("GET", "/translate/shakespeare.json")
        .match_query(Matcher::Regex("text=.*".into()))
        .with_status(200)
        .with_body(
            std::fs::read_to_string("./tests/fixtures/fun_translations_ok_response.json").unwrap(),
        )
        .create();

    let resp = call_get_shakespearean_description_service(pokemon_name).await;

    assert!(resp.status().is_success());
    assert_eq!(
        ShakespeareanDescriptionApiResponse {
            name: pokemon_name.into(),
            description: "A strange seed wast planted on its back at birth. The plant sprouts and grows with this pokémon.".into(),
        },
        test::read_body_json(resp).await
    );
}

#[actix_rt::test]
async fn test_poke_api_returns_status_code_different_from_200() {
    let pokemon_name = "bulbasaur";

    let _poke_api_mock = mock(
        "GET",
        format!("/api/v2/pokemon-species/{}", pokemon_name).as_str(),
    )
    .with_status(404)
    .create();

    let resp = call_get_shakespearean_description_service(pokemon_name).await;

    assert_eq!(404, resp.status());
    assert_eq!(
        JsonErrorResponseBody {
            code: 404,
            message: "HTTP status client error (404 Not Found) for url (http://127.0.0.1:1234/api/v2/pokemon-species/bulbasaur)".into(),
        },
        test::read_body_json(resp).await
    );
}

#[actix_rt::test]
async fn test_poke_apis_returns_200_with_unexpected_body() {
    let pokemon_name = "bulbasaur";

    let _poke_api_mock = mock(
        "GET",
        format!("/api/v2/pokemon-species/{}", pokemon_name).as_str(),
    )
    .with_status(200)
    .with_body("That's the body you're looking for...")
    .create();

    let resp = call_get_shakespearean_description_service(pokemon_name).await;

    assert!(resp.status().is_server_error());
    assert_eq!(
        JsonErrorResponseBody {
            code: 500,
            message: "error decoding response body: expected value at line 1 column 1".into(),
        },
        test::read_body_json(resp).await
    );
}

#[actix_rt::test]
async fn test_poke_apis_returns_200_without_a_traslatable_description() {
    let pokemon_name = "bulbasaur";

    let _poke_api_mock = mock(
        "GET",
        format!("/api/v2/pokemon-species/{}", pokemon_name).as_str(),
    )
    .with_status(200)
    .with_body(
        std::fs::read_to_string(
            "./tests/fixtures/poke_api_not_translatable_description_response.json",
        )
        .unwrap(),
    )
    .create();

    let resp = call_get_shakespearean_description_service(pokemon_name).await;

    assert_eq!(404, resp.status());
    assert_eq!(
        JsonErrorResponseBody {
            code: 404,
            message: "No \'en\' descripiton found when calling PokeApi URL \"http://127.0.0.1:1234/api/v2/pokemon-species/bulbasaur\"".into(),
        },
        test::read_body_json(resp).await
    );
}

#[actix_rt::test]
async fn test_fun_translations_returns_status_code_different_from_200() {
    let pokemon_name = "bulbasaur";

    let _poke_api_mock = mock(
        "GET",
        format!("/api/v2/pokemon-species/{}", pokemon_name).as_str(),
    )
    .with_status(200)
    .with_body(std::fs::read_to_string("./tests/fixtures/poke_api_ok_response.json").unwrap())
    .create();

    let _fun_translations_mock = mock("GET", "/translate/shakespeare.json")
        .match_query(Matcher::Regex("text=.*".into()))
        .with_status(429)
        .with_body(
            std::fs::read_to_string("./tests/fixtures/fun_translations_ok_response.json").unwrap(),
        )
        .create();

    let resp = call_get_shakespearean_description_service(pokemon_name).await;

    assert_eq!(429, resp.status());
    assert_eq!(
        JsonErrorResponseBody {
            code: 429,
            message: "HTTP status client error (429 Too Many Requests) for url (http://127.0.0.1:1234/translate/shakespeare.json?text=A+strange+seed+was+planted+on+its+back+at+birth.+The+plant+sprouts+and+grows+with+this+POK%C3%A9MON.)".into(),
        },
        test::read_body_json(resp).await
    );
}

#[actix_rt::test]
async fn test_fun_translations_returns_200_with_unexpected_body() {
    let pokemon_name = "bulbasaur";

    let _poke_api_mock = mock(
        "GET",
        format!("/api/v2/pokemon-species/{}", pokemon_name).as_str(),
    )
    .with_status(200)
    .with_body(std::fs::read_to_string("./tests/fixtures/poke_api_ok_response.json").unwrap())
    .create();

    let _fun_translations_mock = mock("GET", "/translate/shakespeare.json")
        .match_query(Matcher::Regex("text=.*".into()))
        .with_status(200)
        .with_body("That's the body you're looking for...")
        .create();

    let resp = call_get_shakespearean_description_service(pokemon_name).await;

    assert!(resp.status().is_server_error());
    assert_eq!(
        JsonErrorResponseBody {
            code: 500,
            message: "error decoding response body: expected value at line 1 column 1".into(),
        },
        test::read_body_json(resp).await
    );
}

fn set_up_mocks() -> (PokeApiClient, FunTranslationsClient) {
    let mock_server_url = mockito::server_url();

    std::env::set_var("POKE_API_ENDPOINT", &mock_server_url);
    std::env::set_var("FUN_TRANSLATIONS_API_ENDPOINT", &mock_server_url);

    (
        PokeApiClient::new(&mock_server_url),
        FunTranslationsClient::new(&mock_server_url),
    )
}

async fn call_get_shakespearean_description_service(pokemon_name: &str) -> ServiceResponse {
    let (poke_api_client, fun_translations_client) = set_up_mocks();
    let mut app = test::init_service(App::new().configure(services::config_app)).await;
    let req = TestRequest::get()
        .uri(&format!("/pokemon/{}", pokemon_name))
        .data(poke_api_client)
        .data(fun_translations_client)
        .to_request();
    test::call_service(&mut app, req).await
}
