use rand::prelude::*;
use reqwest::Error as ReqwestError;
use reqwest::Url;
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// HTTP client to interact with PokeApi API.
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

    /// Given a Pokémon name, gets one of its English description randomly.
    ///
    /// In case of no available English descriptions, returns `Err(DescriptionNotFound)`.
    /// In case of any other errors, it transparently returns them.
    /// Note: the descriptions fetched from PokeApi API are filtered by default by "en" language and the randomly picked
    /// one is cleaned from unneeded whitespaces and NULL unicode chars.
    pub async fn get_random_description(
        &self,
        pokemon_name: &str,
    ) -> Result<String, PokeApiClientError> {
        let api_url = format!("{}api/v2/pokemon-species/{}", self.endpoint, pokemon_name);

        let resp = reqwest::get(&api_url)
            .await?
            .error_for_status()?
            .json::<PokemonSpecies>()
            .await?;

        let language_filter = "en";
        let description = Self::pick_random_description(&resp.descriptions, language_filter)
            .ok_or_else(|| {
                PokeApiClientError::DescriptionNotFound(DescriptionNotFound {
                    api_url,
                    language_filter: language_filter.into(),
                })
            })?
            .text
            .as_str();
        Ok(Self::cleanup_description(description))
    }

    fn pick_random_description<'a>(
        descriptions: &'a [PokemonDescription],
        lang: &str,
    ) -> Option<&'a PokemonDescription> {
        descriptions
            .iter()
            .filter(|d| d.language.name == lang)
            .choose(&mut rand::thread_rng())
    }

    fn cleanup_description(description: &str) -> String {
        description
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .replace("\\u000", "")
    }
}

#[derive(Debug)]
pub enum PokeApiClientError {
    DescriptionNotFound(DescriptionNotFound),
    RequestError(ReqwestError),
}

#[derive(Debug)]
pub struct DescriptionNotFound {
    language_filter: String,
    api_url: String,
}

impl StdError for DescriptionNotFound {}

impl Display for DescriptionNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "No '{}' descripiton found when calling PokeApi URL {:?}",
            self.language_filter, self.api_url
        )
    }
}

impl StdError for PokeApiClientError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::DescriptionNotFound(e) => Some(e),
            Self::RequestError(e) => Some(e),
        }
    }
}

impl Display for PokeApiClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::DescriptionNotFound(e) => Display::fmt(e, f),
            Self::RequestError(e) => Display::fmt(e, f),
        }
    }
}

impl From<ReqwestError> for PokeApiClientError {
    fn from(error: ReqwestError) -> Self {
        PokeApiClientError::RequestError(error)
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
