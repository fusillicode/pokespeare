use reqwest::Url;
use serde::Deserialize;

#[derive(Clone)]
pub struct FunTranslationsClient {
    pub endpoint: Url,
}

impl FunTranslationsClient {
    pub async fn translate(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut fun_translations_shakespere_request_url = self.endpoint.clone();
        fun_translations_shakespere_request_url
            .path_segments_mut()
            .map_err(|_| "Can't construct shakespeare.json API URL")?
            .push("shakespeare.json");

        let shakesperean_description_response = reqwest::Client::new()
            .get(fun_translations_shakespere_request_url)
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
