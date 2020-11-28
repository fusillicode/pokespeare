use serde::{Deserialize, Serialize};

/// Response of the `get_shakespearean_description` API service.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ShakespeareanDescriptionApiResponse {
    pub name: String,
    pub description: String,
}
