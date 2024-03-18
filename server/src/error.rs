use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::json;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidArgument(String),
    EntityNotFound(String),
}

#[derive(Debug)]
pub enum ResponseError {
    NotFound(String),
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::NotFound(val) => write!(f, "{}", val),
        }
    }
}

impl actix_web::error::ResponseError for ResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ResponseError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let msg = self.to_string();

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(json!({"error": msg}).to_string())
    }
}

impl From<Error> for ResponseError {
    fn from(value: Error) -> Self {
        match value {
            Error::EntityNotFound(msg) => ResponseError::NotFound(msg),
            Error::InvalidArgument(_) => panic!("not implemented"),
        }
    }
}
