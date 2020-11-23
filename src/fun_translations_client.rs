use reqwest::Error as ReqwestError;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone)]
pub struct FunTranslationsClient {
    pub endpoint: Url,
}

impl FunTranslationsClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: Url::parse(endpoint)
                .unwrap_or_else(|e| panic!("Can't parse {} as URL, error: {:?}", endpoint, e)),
        }
    }

    pub async fn translate(&self, text: &str) -> Result<String, FunTranslationsClientError> {
        let api_url = format!("{}translate/shakespeare.json", self.endpoint);

        let req = Client::new().get(&api_url).query(&[("text", text)]);
        let resp = req.send().await;

        Ok(resp?
            .error_for_status()?
            .json::<ShakespereanDescription>()
            .await?
            .contents
            .translated_text)
    }
}

#[derive(Debug)]
pub struct FunTranslationsClientError(pub ReqwestError);

impl StdError for FunTranslationsClientError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.0)
    }
}

impl Display for FunTranslationsClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<ReqwestError> for FunTranslationsClientError {
    fn from(error: ReqwestError) -> Self {
        FunTranslationsClientError(error)
    }
}

#[derive(Debug, Deserialize)]
struct ShakespereanDescription {
    contents: ShakespereanDescriptionContents,
}

#[derive(Debug, Deserialize)]
struct ShakespereanDescriptionContents {
    #[serde(rename = "translated")]
    translated_text: String,
}
