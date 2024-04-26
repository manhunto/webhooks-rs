use std::fmt::{Display, Formatter};

use itertools::Itertools;
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
    EnabledManually,
}

impl EndpointStatus {
    fn is_active(&self) -> bool {
        match self {
            Self::Initial | Self::EnabledManually => true,
            Self::DisabledManually | Self::DisabledFailing => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: EndpointId,
    pub app_id: ApplicationId,
    pub url: Url,
    pub topics: TopicsList,
    pub status: EndpointStatus,
}

impl Endpoint {
    pub fn new(url: String, app_id: ApplicationId, topics: TopicsList) -> Self {
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

    pub fn enable_manually(&mut self) {
        self.status = EndpointStatus::EnabledManually;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Topic {
    name: String,
}

impl Topic {
    pub fn new<T>(name: T) -> Result<Self, Error>
    where
        T: AsRef<str>,
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_\.\-]+$").unwrap();
        }

        if !RE.is_match(name.as_ref()) {
            return Err(InvalidArgument("Invalid topic name".to_string()));
        }

        Ok(Self {
            name: name.as_ref().to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopicsList {
    topics: Vec<Topic>,
}

impl TopicsList {
    pub fn contains(&self, topic: &Topic) -> bool {
        self.topics.contains(topic)
    }
}

impl TryFrom<Vec<String>> for TopicsList {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(InvalidArgument(
                "Topic collection could not be empty".to_string(),
            ));
        }

        let topics: Vec<Topic> = value.iter().map(Topic::new).try_collect()?;

        Ok(Self { topics })
    }
}

impl FromIterator<String> for TopicsList {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut vec = Vec::new();
        for v in iter {
            vec.push(v);
        }

        TopicsList::try_from(vec).unwrap()
    }
}

impl From<TopicsList> for Vec<String> {
    fn from(value: TopicsList) -> Self {
        value.topics.into_iter().map(|t| t.name).collect()
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod endpoint_tests {
    use crate::configuration::domain::{ApplicationId, Endpoint, TopicsList};

    #[test]
    fn endpoint_disable_manually_is_not_active() {
        let mut endpoint = EndpointObjectMother::init_new();
        assert!(endpoint.is_active());

        endpoint.disable_manually();
        assert!(!endpoint.is_active());
    }

    #[test]
    fn endpoint_disable_failing_is_not_active() {
        let mut endpoint = EndpointObjectMother::init_new();
        assert!(endpoint.is_active());

        endpoint.disable_failing();
        assert!(!endpoint.is_active());
    }

    #[test]
    fn endpoint_enable_manually_is_active() {
        let mut endpoint = EndpointObjectMother::init_disabled();
        assert!(!endpoint.is_active());

        endpoint.enable_manually();
        assert!(endpoint.is_active());
    }

    struct EndpointObjectMother;

    impl EndpointObjectMother {
        fn init_new() -> Endpoint {
            Endpoint::new(
                "https://localhost".to_string(),
                ApplicationId::new(),
                TopicsList::try_from(vec![String::from("test")]).unwrap(),
            )
        }

        fn init_disabled() -> Endpoint {
            let mut endpoint = Self::init_new();
            endpoint.disable_manually();

            endpoint
        }
    }
}

#[cfg(test)]
mod topic_tests {
    use crate::configuration::domain::Topic;
    use crate::tests::assert_strings;

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
        assert_strings!("order.purchased", |str| Topic::new(str).is_ok());
    }
}

#[cfg(test)]
mod topics_list_tests {
    use crate::configuration::domain::TopicsList;
    use crate::error::Error::InvalidArgument;

    #[test]
    fn cannot_be_empty() {
        let sut = TopicsList::try_from(vec![]);

        assert_eq!(
            Err(InvalidArgument(
                "Topic collection could not be empty".to_string()
            )),
            sut
        );
    }

    #[test]
    fn cannot_be_build_with_invalid_topics_name() {
        let topics = vec![
            String::from("contact.updated"),
            String::from("contact.updated2"),
        ];
        let sut = TopicsList::try_from(topics);

        assert!(sut.is_err());
    }

    #[test]
    fn can_be_build_with_valid_topic_names() {
        let topics = vec![
            String::from("contact.updated"),
            String::from("contact.created"),
        ];
        let sut = TopicsList::try_from(topics);

        assert!(sut.is_ok());
    }
}
