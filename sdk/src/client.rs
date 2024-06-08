use std::path::PathBuf;
use std::str::FromStr;

use reqwest::header;
use reqwest::header::USER_AGENT;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::error::Error;
use crate::error::Error::BadRequest;

#[derive(Clone)]
pub struct Client {
    base_url: Url,
    client: reqwest::Client,
}

impl Client {
    pub fn new(api_url: Url) -> Self {
        Self {
            base_url: api_url,
            client: Self::client(),
        }
    }

    pub async fn post<I, O>(&self, endpoint: EndpointUrl, body: I) -> Result<O, Error>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let url = self.url(endpoint);
        let response = self.client.post(url).json(&body).send().await?;

        if 400 == response.status().as_u16() {
            let result = response.json::<crate::error::BadRequest>().await?;

            return Err(BadRequest(result));
        }

        Ok(response.json::<O>().await?)
    }

    fn url(&self, endpoint: EndpointUrl) -> Url {
        self.base_url.join(endpoint.as_str()).unwrap_or_else(|_| {
            panic!(
                "Could not join strings to create endpoint url: '{}', '{}'",
                self.base_url,
                endpoint.as_str()
            )
        })
    }

    fn client() -> reqwest::Client {
        let mut headers = header::HeaderMap::new();
        let sdk_version = env!("CARGO_PKG_VERSION");

        headers.insert(
            USER_AGENT,
            header::HeaderValue::from_str(
                format!("webhooks-rs rust sdk v{}", sdk_version).as_str(),
            )
            .unwrap(),
        );

        reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct EndpointUrl {
    path: PathBuf, // fixme: it won't work on windows
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

impl TryFrom<String> for EndpointUrl {
    type Error = Self;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self::new(value))
    }
}
