mod application;
mod client;

use crate::application::Application;
use client::Client;
use url::Url;

pub struct WebhooksSDK {
    client: Client,
}

impl WebhooksSDK {
    pub fn new(api_url: String) -> Self {
        let url = Url::parse(api_url.as_str()).unwrap();

        Self {
            client: Client::new(url),
        }
    }

    pub fn application(&self) -> Application {
        Application::new(self.client.clone())
    }
}
