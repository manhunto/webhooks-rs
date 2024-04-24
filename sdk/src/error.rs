use std::fmt::{Display, Formatter};

use crate::error::Error::{Reqwest, Unknown};

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Unknown,
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Reqwest(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Reqwest(err) => format!("{}", err),
            Unknown => "Unknown error".to_string(),
        };

        write!(f, "{}", val)
    }
}

impl std::error::Error for Error {}
