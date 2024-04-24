use std::path::PathBuf;
use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::error::Error;

#[derive(Clone)]
pub struct Client {
    api_url: Url,
}

impl Client {
    pub fn new(api_url: Url) -> Self {
        Self { api_url }
    }

    pub async fn post<I, O>(&self, endpoint: EndpointUrl, body: I) -> Result<O, Error>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let url = self.api_url.join(endpoint.as_str()).unwrap_or_else(|_| {
            panic!(
                "Could not join strings to create endpoint url: '{}', '{}'",
                self.api_url,
                endpoint.as_str()
            )
        });

        let response = reqwest::Client::new().post(url).json(&body).send().await?;

        Ok(response.json::<O>().await?)
    }
}

#[derive(Debug)]
pub struct EndpointUrl {
    path: PathBuf,
}

impl EndpointUrl {
    #[must_use]
    pub fn new(path: String) -> Self {
        let path_buf = PathBuf::from(path);

        Self { path: path_buf }
    }

    fn as_str(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

impl FromStr for EndpointUrl {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
