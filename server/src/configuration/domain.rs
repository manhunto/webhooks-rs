use std::fmt::{Display, Formatter};

use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

use crate::error::Error;
use crate::error::Error::InvalidArgument;

#[derive(Debug, Clone, derive::Ksuid, Eq, PartialEq)]
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

#[derive(Debug, Clone, derive::Ksuid, Eq, PartialEq)]
#[prefix = "ep"]
pub struct EndpointId {
    id: String,
}

#[derive(Clone, Debug)]
pub enum EndpointStatus {
    Initial,
    DisabledManually,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: EndpointId,
    pub app_id: ApplicationId,
    pub url: Url,
    pub topics: Vec<Topic>,
    pub status: EndpointStatus,
}

impl Endpoint {
    pub fn new(url: String, app_id: ApplicationId, topics: Vec<Topic>) -> Self {
        Self {
            id: EndpointId::new(),
            url: Url::parse(url.as_str()).unwrap(),
            topics,
            app_id,
            status: EndpointStatus::Initial,
        }
    }

    pub fn disable_manually(&mut self) {
        self.status = EndpointStatus::DisabledManually;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Topic {
    name: String,
}

impl Topic {
    pub fn new(name: String) -> Result<Self, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_\.\-]+$").unwrap();
        }

        if !RE.is_match(name.as_str()) {
            return Err(InvalidArgument("Invalid topic name".to_string()));
        }

        Ok(Self { name })
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::configuration::domain::Topic;

    #[test]
    fn topic_name_construct() {
        assert!(Topic::new("customer_updated".to_string()).is_ok());
        assert!(Topic::new("customer-updated".to_string()).is_ok());
        assert!(Topic::new("customer.updated".to_string()).is_ok());
        assert!(Topic::new("customer.updated2".to_string()).is_err());
    }
}
