use std::str::FromStr;

use serde::Deserialize;
use serde_json::json;

use crate::client::{Client, EndpointUrl};
use crate::error::Error;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Application {
    pub id: String,
    pub name: String,
}

pub struct ApplicationApi {
    client: Client,
}

impl ApplicationApi {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, name: &str) -> Result<Application, Error> {
        let body = json!({
            "name": name,
        });

        self.client
            .post(EndpointUrl::from_str("application").unwrap(), body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher::Json;
    use serde_json::json;

    use crate::application::Application;
    use crate::error::Error;
    use crate::WebhooksSDK;

    #[tokio::test]
    async fn create_application() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("POST", "/application")
            .match_body(Json(json!({"name": "dummy application"})))
            .with_body(r#"{"id":"app_2dSZgxc6qw0vR7hwZVXDJFleRXj","name":"dummy application"}"#)
            .with_header("content-type", "application/json")
            .with_status(201)
            .create_async()
            .await;

        let app = WebhooksSDK::new(url.as_str())
            .application()
            .create("dummy application")
            .await
            .unwrap();

        mock.assert_async().await;

        assert_eq!(
            Application {
                id: "app_2dSZgxc6qw0vR7hwZVXDJFleRXj".to_string(),
                name: "dummy application".to_string(),
            },
            app
        );
    }

    #[tokio::test]
    async fn can_handle_bad_request() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        server
            .mock("POST", "/application")
            .match_body(Json(json!({"name": ""})))
            .with_body(r#"{"error":"Validation error","messages":["Name cannot be empty"]}"#)
            .with_header("content-type", "application/json")
            .with_status(400)
            .create_async()
            .await;

        let error = WebhooksSDK::new(url.as_str())
            .application()
            .create("")
            .await
            .err()
            .unwrap();

        match error {
            Error::Reqwest(req) => panic!("is reqwest error {}", req),
            Error::Unknown => panic!("is unknown error"),
            Error::BadRequest(br) => {
                assert_eq!("Validation error", br.error());
                assert_eq!(vec!["Name cannot be empty"], br.messages());
            }
        }
    }
}
