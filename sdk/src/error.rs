use thiserror::Error;

use crate::error::Error::Reqwest;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error occurred during request: {0}")]
    Reqwest(reqwest::Error),
    #[error("Unknown error")]
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Reqwest(value)
    }
}
