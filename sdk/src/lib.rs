use url::Url;

use client::Client;

use crate::application::ApplicationApi;
use crate::endpoint::EndpointApi;

mod application;
mod client;
mod endpoint;
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

    pub fn application(&self) -> ApplicationApi {
        ApplicationApi::new(self.client.clone())
    }

    pub fn endpoints(&self) -> EndpointApi {
        EndpointApi::new(self.client.clone())
    }
}
