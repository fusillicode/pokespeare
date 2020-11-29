use crate::fun_translations_client::FunTranslationsClientError;
use crate::poke_api_client::PokeApiClientError;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use reqwest::StatusCode as ReqwestStatusCode;
use serde::{Deserialize, Serialize};

/// Representation of an API error response body.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiErrorResponseBody {
    pub code: ApiErrorResponseCode,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiErrorResponseCode {
    TranslatableDescriptionNotFound,
    PokeApiError,
    FunTranslationsError,
    TooManyRequests,
}

/// Make `PokeApiClientError` an `actix_web` "citizen" by implementing `actix_web::error::ResponseError`.
impl ResponseError for PokeApiClientError {
    fn status_code(&self) -> StatusCode {
        match self {
            PokeApiClientError::TraslatableDescriptionNotFound(_) => StatusCode::NOT_FOUND,
            PokeApiClientError::RequestError(e) => map_reqwest_to_actix_status_code(e.status()),
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            PokeApiClientError::TraslatableDescriptionNotFound(e) => {
                HttpResponse::NotFound().json(ApiErrorResponseBody {
                    code: ApiErrorResponseCode::TranslatableDescriptionNotFound,
                    message: e.to_string(),
                })
            }
            PokeApiClientError::RequestError(e) => HttpResponse::build(
                map_reqwest_to_actix_status_code(e.status()),
            )
            .json(ApiErrorResponseBody {
                code: ApiErrorResponseCode::PokeApiError,
                message: e.to_string(),
            }),
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
            Some(StatusCode::TOO_MANY_REQUESTS) => HttpResponse::build(
                map_reqwest_to_actix_status_code(Some(StatusCode::TOO_MANY_REQUESTS)),
            )
            .json(ApiErrorResponseBody {
                code: ApiErrorResponseCode::TooManyRequests,
                message: self.0.to_string(),
            }),
            Some(status_code) => HttpResponse::build(map_reqwest_to_actix_status_code(Some(
                status_code,
            )))
            .json(ApiErrorResponseBody {
                code: ApiErrorResponseCode::FunTranslationsError,
                message: self.0.to_string(),
            }),
            None => HttpResponse::InternalServerError().json(ApiErrorResponseBody {
                code: ApiErrorResponseCode::FunTranslationsError,
                message: self.0.to_string(),
            }),
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
