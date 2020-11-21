use reqwest::Url;
use serde::Deserialize;

#[derive(Clone)]
pub struct FunTranslationsClient {
    pub endpoint: Url,
}

impl FunTranslationsClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: Url::parse(endpoint)
                .unwrap_or_else(|e| panic!("Can't parse {} as URL, error: {:?}", endpoint, e))
        }
    }

    pub async fn translate(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_url = format!("{}/shakespeare.json", self.endpoint);

        let shakesperean_description_response = reqwest::Client::new()
            .get(&api_url)
            .query(&[("text", text)])
            .send()
            .await;

        Ok(shakesperean_description_response?
            .json::<ShakespereanDescription>()
            .await?
            .contents
            .translated_text)
    }
}

#[derive(Deserialize)]
struct ShakespereanDescription {
    contents: ShakespereanDescriptionContents,
}

#[derive(Deserialize)]
struct ShakespereanDescriptionContents {
    #[serde(rename = "translated")]
    translated_text: String,
}
