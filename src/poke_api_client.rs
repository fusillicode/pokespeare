use rand::prelude::*;
use reqwest::Url;
use serde::Deserialize;

#[derive(Clone)]
pub struct PokeApiClient {
    pub endpoint: Url,
}

impl PokeApiClient {
    pub async fn get_random_description(
        &self,
        pokemon_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut poke_api_species_request_url = self.endpoint.clone();
        poke_api_species_request_url
            .path_segments_mut()
            .map_err(|_| "Can't construct pokemon-species API URL")?
            .extend(&["pokemon-species", &pokemon_name]);

        let pokemon_species = reqwest::get(poke_api_species_request_url.clone())
            .await?
            .json::<PokemonSpecies>()
            .await?;

        let language_filter = "en";
        Ok(pokemon_species
            .descriptions
            .iter()
            .filter(|d| d.language.name == language_filter)
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| {
                format!(
                    "No '{}' descripiton found when calling PokeApi URL {:?}",
                    language_filter, poke_api_species_request_url
                )
            })?
            .text
            .replace('\n', " ")
            .replace("\\u000", ""))
    }
}

#[derive(Deserialize)]
struct PokemonSpecies {
    #[serde(rename = "flavor_text_entries")]
    descriptions: Vec<PokemonDescription>,
}

#[derive(Deserialize)]
struct PokemonDescription {
    #[serde(rename = "flavor_text")]
    text: String,
    language: Language,
}

#[derive(Deserialize)]
struct Language {
    name: String,
}
