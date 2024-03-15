use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Debug, Clone, derive::Ksuid)]
#[prefix = "app"]
pub struct ApplicationId {
    id: String,
}

#[derive(Debug, Clone)]
pub struct Application {
    pub id: ApplicationId,
    pub name: String,
}

impl Application {
    pub fn new(name: String) -> Self {
        Self {
            id: ApplicationId::new(),
            name,
        }
    }
}

#[derive(Debug, Clone, derive::Ksuid)]
#[prefix = "ep"]
pub struct EndpointId {
    id: String,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: EndpointId,
    pub app_id: ApplicationId,
    pub url: Url,
}

impl Endpoint {
    pub fn new(url: String, app_id: ApplicationId) -> Self {
        Self {
            id: EndpointId::new(),
            url: Url::parse(url.as_str()).unwrap(),
            app_id,
        }
    }
}
