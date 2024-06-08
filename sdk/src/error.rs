use serde::Deserialize;
use thiserror::Error;

use crate::error::Error::Reqwest;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error occurred during request: {0}")]
    Reqwest(reqwest::Error),
    #[error("Unknown error")]
    Unknown,
    #[error("Bad request")]
    BadRequest(BadRequest),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Reqwest(value)
    }
}

#[derive(Deserialize, Debug)]
pub struct BadRequest {
    error: String,
    messages: Vec<String>,
}

impl BadRequest {
    pub fn error(&self) -> String {
        self.error.clone()
    }

    pub fn messages(&self) -> Vec<String> {
        self.messages.clone()
    }
}
