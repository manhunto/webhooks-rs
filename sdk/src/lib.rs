use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

pub struct WebhooksSDK {
    api_url: String,
}

impl WebhooksSDK {
    #[allow(dead_code)]
    fn new(api_url: String) -> Self {
        Self { api_url }
    }

    fn application(&self) -> Application {
        Application {
            client: Client::new(self.api_url.to_string()),
        }
    }
}

struct Client {
    api_url: String,
}

impl Client {
    fn new(api_url: String) -> Self {
        Self { api_url }
    }

    async fn post<I, O>(&self, url: String, body: I) -> O
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let url = format!("{}/{}", self.api_url, url);

        let response = reqwest::Client::new()
            .post(url)
            .json(&body)
            .send()
            .await
            .unwrap(); // todo handle errors

        response.json::<O>().await.unwrap() // todo handle errors
    }
}

struct Application {
    client: Client,
}

#[derive(Deserialize, Debug, PartialEq)]
struct App {
    id: Uuid,
    name: String,
}

impl Application {
    pub async fn create(&self, name: String) -> App {
        let body = json!({
            "name": name,
        });
        let url = "v1/application";
        let app: App = self.client.post(url.to_string(), body).await;

        app
    }
}

#[cfg(test)]
mod tests {
    use crate::{App, WebhooksSDK};
    use mockito::Matcher::Json;
    use serde_json::json;
    use uuid::uuid;

    #[tokio::test]
    async fn create_application() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock("POST", "/v1/application")
            .match_body(Json(json!({"name": "dummy application"})))
            .with_body(
                r#"{"id":"78986a6c-b1ba-4729-8fae-b080e5f91551","name":"dummy application"}"#,
            )
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
                id: uuid!("78986a6c-b1ba-4729-8fae-b080e5f91551"),
                name: "dummy application".to_string()
            },
            app
        );
    }
}
