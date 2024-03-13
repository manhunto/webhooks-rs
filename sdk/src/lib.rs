mod application;
mod client;

use crate::application::Application;
use client::Client;
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
        Application::new(Client::new(self.api_url.clone()))
    }
}
