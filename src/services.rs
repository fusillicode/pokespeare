use crate::fun_translations_client::FunTranslationsClient;
use crate::poke_api_client::PokeApiClient;
use crate::services_api_models::ShakespereanDescriptionApiResponse;
use actix_web::web::{Data, Path, ServiceConfig};
use actix_web::{get, Error, HttpResponse};

pub fn config_app(cfg: &mut ServiceConfig) {
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
        .await?;

    let shakesperean_description = fun_translations_client
        .translate(&pokemon_description)
        .await?;

    Ok(HttpResponse::Ok().json(ShakespereanDescriptionApiResponse {
        name: pokemon_name.to_string(),
        description: shakesperean_description,
    }))
}
