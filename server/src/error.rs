use std::fmt::{Display, Formatter};

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::json;
use sqlx::Error as SqlxError;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidArgument(String),
    EntityNotFound(String),
    Sqlx(String),
}

#[derive(Debug)]
pub enum ResponseError {
    NotFound(String),
    BadRequest(String),
    InternalError,
    ValidationError(ValidationErrors),
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ResponseError::NotFound(val) | ResponseError::BadRequest(val) => val,
            ResponseError::InternalError => "",
            ResponseError::ValidationError(_) => "Validation errors",
        };

        write!(f, "{msg}")
    }
}

impl actix_web::error::ResponseError for ResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ResponseError::NotFound(_) => StatusCode::NOT_FOUND,
            ResponseError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ResponseError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error = self.to_string();

        let messages: Vec<String> = match self {
            ResponseError::NotFound(_)
            | ResponseError::BadRequest(_)
            | ResponseError::InternalError => Vec::<String>::new(),
            ResponseError::ValidationError(errors) => {
                let inner: Vec<Vec<String>> = errors
                    .errors()
                    .iter()
                    .map(|e| match e.1 {
                        ValidationErrorsKind::Field(err) => {
                            err.iter().map(|e| e.to_string()).collect()
                        }
                        _ => unreachable!("this is error type is not handled yet"),
                    })
                    .collect();

                inner.into_iter().flatten().collect()
            }
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(
                json!({
                    "error": error,
                    "messages": messages
                })
                .to_string(),
            )
    }
}

impl From<Error> for ResponseError {
    fn from(value: Error) -> Self {
        match value {
            Error::EntityNotFound(msg) => ResponseError::NotFound(msg),
            Error::InvalidArgument(msg) => ResponseError::BadRequest(msg),
            Error::Sqlx(_) => ResponseError::InternalError,
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: SqlxError) -> Self {
        match value {
            SqlxError::RowNotFound => Self::EntityNotFound("Entity not found".to_string()),
            _ => Self::Sqlx(value.to_string()),
        }
    }
}
