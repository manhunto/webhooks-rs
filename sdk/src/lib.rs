use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

pub struct WebhooksSDK {
    api_url: Url,
}

impl WebhooksSDK {
    pub fn new(api_url: String) -> Self {
        Self {
            api_url: Url::parse(api_url.as_str()).unwrap(),
        }
    }

    pub fn application(&self) -> Application {
        Application {
            client: Client::new(self.api_url.clone()),
        }
    }
}

struct Client {
    api_url: Url,
}

impl Client {
    fn new(api_url: Url) -> Self {
        Self { api_url }
    }

    async fn post<I, O>(&self, endpoint: EndpointUrl, body: I) -> O
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let url = self.api_url.join(endpoint.as_str()).expect("Invalid url");

        let response = reqwest::Client::new()
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap(); // todo handle errors

        response.json::<O>().await.unwrap() // todo handle errors
    }
}

pub struct Application {
    client: Client,
}

#[derive(Debug)]
struct EndpointUrl {
    path: PathBuf,
}

impl EndpointUrl {
    #[must_use]
    fn new(path: String) -> Self {
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

#[derive(Deserialize, Debug, PartialEq)]
pub struct App {
    id: String,
    name: String,
}

impl Application {
    pub async fn create(&self, name: String) -> App {
        let body = json!({
            "name": name,
        });

        self.client
            .post(EndpointUrl::from_str("v1/application").unwrap(), body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{App, WebhooksSDK};
    use mockito::Matcher::Json;
    use serde_json::json;

    #[tokio::test]
    async fn create_application() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("POST", "/v1/application")
            .match_body(Json(json!({"name": "dummy application"})))
            .with_body(r#"{"id":"app_2dSZgxc6qw0vR7hwZVXDJFleRXj","name":"dummy application"}"#)
            .with_header("content-type", "application/json")
            .with_status(201)
            .create_async()
            .await;

        let app = WebhooksSDK::new(url)
            .application()
            .create("dummy application".to_string())
            .await;

        mock.assert_async().await;

        assert_eq!(
            App {
                id: "app_2dSZgxc6qw0vR7hwZVXDJFleRXj".to_string(),
                name: "dummy application".to_string()
            },
            app
        );
    }
}
