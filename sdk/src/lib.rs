use url::Url;

use client::Client;

use crate::application::Application;

mod application;
mod client;
pub mod error;

pub struct WebhooksSDK {
    client: Client,
}

impl WebhooksSDK {
    pub fn new(api_url: &str) -> Self {
        let url = Url::parse(api_url).unwrap();

        Self {
            client: Client::new(url),
        }
    }

    pub fn application(&self) -> Application {
        Application::new(self.client.clone())
    }
}
