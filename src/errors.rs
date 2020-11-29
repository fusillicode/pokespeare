use crate::fun_translations_client::FunTranslationsClientError;
use crate::poke_api_client::PokeApiClientError;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode as ReqwestStatusCode;
use serde::{Deserialize, Serialize};

/// Representation of an error response body.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonErrorResponseBody {
    pub code: u16,
    pub message: String,
}

impl JsonErrorResponseBody {
    fn new(err: &ReqwestError) -> Self {
        Self {
            code: map_reqwest_to_actix_status_code(err.status()).as_u16(),
            message: err.to_string(),
        }
    }
}

/// Make `PokeApiClientError` an `actix_web` "citizen" by implementing `actix_web::error::ResponseError`.
impl ResponseError for PokeApiClientError {
    fn status_code(&self) -> StatusCode {
        match self {
            PokeApiClientError::DescriptionNotFound(_) => StatusCode::NOT_FOUND,
            PokeApiClientError::RequestError(e) => map_reqwest_to_actix_status_code(e.status()),
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            PokeApiClientError::DescriptionNotFound(e) => {
                HttpResponse::NotFound().json(JsonErrorResponseBody {
                    code: 404,
                    message: e.to_string(),
                })
            }
            PokeApiClientError::RequestError(e) => {
                HttpResponse::build(map_reqwest_to_actix_status_code(e.status()))
                    .json(JsonErrorResponseBody::new(&e))
            }
        }
    }
}

/// Make `FunTranslationsClientError` an `actix_web` "citizen" by implementing `actix_web::error::ResponseError`.
impl ResponseError for FunTranslationsClientError {
    fn status_code(&self) -> StatusCode {
        match self.0.status() {
            Some(status_code) => map_reqwest_to_actix_status_code(Some(status_code)),
            None => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self.0.status() {
            Some(status_code) => {
                HttpResponse::build(map_reqwest_to_actix_status_code(Some(status_code)))
                    .json(JsonErrorResponseBody::new(&self.0))
            }
            None => HttpResponse::InternalServerError().json(JsonErrorResponseBody::new(&self.0)),
        }
    }
}

/// Utility to convert an optional `reqwest::StatusCode` into a `actix_web::http::StatusCode`.
/// With no input status code, returns a `actix_web::http::StatusCode::INTERNAL_SERVER_ERROR`.
///
/// Panics if it can't get a valid `actix_web::http::StatusCode`.
fn map_reqwest_to_actix_status_code(reqwest_status_code: Option<ReqwestStatusCode>) -> StatusCode {
    reqwest_status_code.map(|s|
        StatusCode::from_u16(s.as_u16()).unwrap_or_else(|e| {
            panic!(
                "Can't build actix_web::http::StatusCode from reqwest::StatusCode {:?}, error: {:?}",
                s, e
            )
        })
    ).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
