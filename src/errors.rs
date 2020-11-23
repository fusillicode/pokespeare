use crate::fun_translations_client::FunTranslationsClientError;
use crate::poke_api_client::PokeApiClientError;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use reqwest::StatusCode as ReqwestStatusCode;

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
                    .body(self.0.to_string())
            }
            None => HttpResponse::InternalServerError().body(self.0.to_string()),
        }
    }
}

impl ResponseError for PokeApiClientError {
    fn status_code(&self) -> StatusCode {
        match self {
            PokeApiClientError::DescriptionNotFound(_) => StatusCode::NOT_FOUND,
            PokeApiClientError::RequestError(e) => map_reqwest_to_actix_status_code(e.status()),
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            PokeApiClientError::DescriptionNotFound(_) => {
                HttpResponse::NotFound().body(self.to_string())
            }
            PokeApiClientError::RequestError(e) => {
                HttpResponse::build(map_reqwest_to_actix_status_code(e.status()))
                    .body(self.to_string())
            }
        }
    }
}

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
