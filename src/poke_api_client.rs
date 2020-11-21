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
        let api_url = format!("{}/pokemon-species/{}", self.endpoint, pokemon_name);

        let pokemon_species = reqwest::get(&api_url)
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
                    language_filter, api_url
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
