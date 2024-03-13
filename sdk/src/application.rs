use crate::client::{Client, EndpointUrl};
use serde::Deserialize;
use serde_json::json;
use std::str::FromStr;

#[derive(Deserialize, Debug, PartialEq)]
pub struct App {
    id: String,
    name: String,
}

pub struct Application {
    client: Client,
}

impl Application {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

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
    use crate::application::App;
    use crate::WebhooksSDK;
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
