use reqwest::Error as ReqwestError;
use reqwest::{Client, Url};
use serde::Deserialize;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// HTTP client to interact with FunTranslations API.
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

    /// Given a text, gets the shakespearean translation by calling FunTranslation API.
    ///
    /// In case of errors, it transparently returns them.
    /// Note: the called FunTranslation API is throttled and returns an error and a status code of 429 in case of too
    /// many requests (at the time of writing the limits are 5 requests per hour).
    pub async fn translate(&self, text: &str) -> Result<String, FunTranslationsClientError> {
        let api_url = format!("{}translate/shakespeare.json", self.endpoint);

        let req = Client::new().get(&api_url).query(&[("text", text)]);
        let resp = req.send().await;

        Ok(resp?
            .error_for_status()?
            .json::<ShakespeareanDescription>()
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
struct ShakespeareanDescription {
    contents: ShakespeareanDescriptionContents,
}

#[derive(Debug, Deserialize)]
struct ShakespeareanDescriptionContents {
    #[serde(rename = "translated")]
    translated_text: String,
}
