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

#[derive(Deserialize)]
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
