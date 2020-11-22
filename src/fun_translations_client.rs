use reqwest::{Client, Url};
use serde::Deserialize;

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

    pub async fn translate(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = format!("{}translate/shakespeare.json", self.endpoint);

        let req = Client::new().get(&api_url).query(&[("text", text)]);
        let resp = req.send().await;

        Ok(resp?
            .json::<ShakespereanDescription>()
            .await?
            .contents
            .translated_text)
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
