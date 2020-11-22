use rand::prelude::*;
use reqwest::Url;
use serde::Deserialize;

#[derive(Clone)]
pub struct PokeApiClient {
    endpoint: Url,
}

impl PokeApiClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: Url::parse(endpoint)
                .unwrap_or_else(|e| panic!("Can't parse {} as URL, error: {:?}", endpoint, e)),
        }
    }

    pub async fn get_random_description(
        &self,
        pokemon_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = format!("{}api/v2/pokemon-species/{}", self.endpoint, pokemon_name);

        let resp = reqwest::get(&api_url)
            .await?
            .json::<PokemonSpecies>()
            .await?;

        let language_filter = "en";
        Ok(resp
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
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .replace("\\u000", ""))
    }
}

#[derive(Debug, Deserialize)]
struct PokemonSpecies {
    #[serde(rename = "flavor_text_entries")]
    descriptions: Vec<PokemonDescription>,
}

#[derive(Debug, Deserialize)]
struct PokemonDescription {
    #[serde(rename = "flavor_text")]
    text: String,
    language: Language,
}

#[derive(Debug, Deserialize)]
struct Language {
    name: String,
}
