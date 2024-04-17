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
    DisabledFailing,
}

impl EndpointStatus {
    fn is_active(&self) -> bool {
        match self {
            Self::Initial => true,
            Self::DisabledManually | Self::DisabledFailing => false,
        }
    }
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

    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }

    pub fn disable_manually(&mut self) {
        self.status = EndpointStatus::DisabledManually;
    }

    pub fn disable_failing(&mut self) {
        self.status = EndpointStatus::DisabledManually;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Topic {
    name: String,
}

impl Topic {
    pub fn new<T>(name: T) -> Result<Self, Error>
        where T: AsRef<str>
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_\.\-]+$").unwrap();
        }

        if !RE.is_match(name.as_ref()) {
            return Err(InvalidArgument("Invalid topic name".to_string()));
        }

        Ok(Self { name: name.as_ref().to_string() })
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod endpoint_tests {
    use crate::configuration::domain::{ApplicationId, Endpoint, Topic};

    #[test]
    fn endpoint_disable_manually_is_active() {
        let mut endpoint = EndpointObjectMother::init_new();
        assert!(endpoint.is_active());

        endpoint.disable_manually();
        assert!(!endpoint.is_active());
    }

    #[test]
    fn endpoint_disable_is_active() {
        let mut endpoint = EndpointObjectMother::init_new();
        assert!(endpoint.is_active());

        endpoint.disable_manually();
        assert!(!endpoint.is_active());
    }

    struct EndpointObjectMother;

    impl EndpointObjectMother {
        fn init_new() -> Endpoint {
            Endpoint::new(
                "https://localhost".to_string(),
                ApplicationId::new(),
                vec![Topic::new("test").unwrap()],
            )
        }
    }
}

#[cfg(test)]
mod topic_tests {
    use crate::configuration::domain::Topic;

    #[test]
    fn topic_name_construct() {
        assert!(Topic::new("customer_updated").is_ok());
        assert!(Topic::new("customer-updated").is_ok());
        assert!(Topic::new("customer.updated").is_ok());
        assert!(Topic::new("customer.updated2").is_err());
        assert!(Topic::new("customer updated").is_err());
        assert!(Topic::new("").is_err());
        assert!(Topic::new(" ").is_err());
    }

    #[test]
    fn topic_can_be_build_from_any_type_of_str() {
        let a: &str = "order.purchased";
        let b: String = String::from("order.purchased");
        let c: &String = &b;

        assert!(Topic::new(a).is_ok());
        assert!(Topic::new(c).is_ok());
        assert!(Topic::new(b).is_ok());
    }
}
