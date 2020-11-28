use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ShakespereanDescriptionApiResponse {
    pub name: String,
    pub description: String,
}
