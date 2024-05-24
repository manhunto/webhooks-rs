use std::fmt::{Display, Formatter};
use std::vec::IntoIter;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::postgres::PgRow;
use sqlx::types::JsonValue;
use sqlx::{FromRow, Row};
use url::Url;

use crate::error::Error;
use crate::error::Error::InvalidArgument;
use crate::types::{ApplicationId, EndpointId};

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

#[derive(Clone, Debug, PartialEq)]
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

impl Display for EndpointStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EndpointStatus::Initial => "initial",
            EndpointStatus::DisabledManually => "disabled_manually",
            EndpointStatus::DisabledFailing => "disabled_failing",
            EndpointStatus::EnabledManually => "enabled_manually",
        };

        write!(f, "{}", str)
    }
}

impl TryFrom<String> for EndpointStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            // fixme: why I need to trim it?
            "initial" => Ok(EndpointStatus::Initial),
            "disabled_manually" => Ok(EndpointStatus::DisabledManually),
            "disabled_failing" => Ok(EndpointStatus::DisabledFailing),
            "enabled_manually" => Ok(EndpointStatus::EnabledManually),
            _ => Err(format!("Unexpected endpoint status: {}", value)),
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

impl FromRow<'_, PgRow> for Endpoint {
    fn from_row(row: &'_ PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let app_id: String = row.try_get("app_id")?;
        let url: String = row.try_get("url")?;
        let status: String = row.try_get("status")?;
        let topics: JsonValue = row.try_get("topics")?;

        let topics: Vec<String> = topics
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t.as_str().unwrap().to_string())
            .collect();

        Ok(Endpoint {
            id: EndpointId::try_from(format!("ep_{}", id)).unwrap(), // fixme: without adding prefix
            app_id: ApplicationId::try_from(format!("app_{}", app_id)).unwrap(), // fixme: without adding prefix
            url: Url::parse(&url).unwrap(),
            topics: TopicsList::try_from(topics).unwrap(),
            status: EndpointStatus::try_from(status.trim().to_string()).unwrap(),
        })
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

impl TryFrom<String> for Topic {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Topic {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopicsList {
    topics: Vec<Topic>,
}

impl TopicsList {
    pub fn new(topics: Vec<Topic>) -> Result<Self, Error> {
        if topics.is_empty() {
            return Err(InvalidArgument(
                "Topic collection could not be empty".to_string(),
            ));
        }

        Ok(Self { topics })
    }

    pub fn contains(&self, topic: &Topic) -> bool {
        self.topics.contains(topic)
    }

    pub fn as_strings(&self) -> Vec<String> {
        self.topics.clone().into_iter().map(|t| t.name).collect()
    }
}

impl TryFrom<Vec<String>> for TopicsList {
    type Error = Error;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let topics: Vec<Topic> = value.iter().map(Topic::new).try_collect()?;

        Self::new(topics)
    }
}

impl From<Vec<&'static str>> for TopicsList {
    fn from(value: Vec<&'static str>) -> Self {
        let vec: Vec<String> = value.into_iter().map(|s| s.to_string()).collect();

        Self::try_from(vec).unwrap()
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
        value.into_iter().map(|t| t.name).collect()
    }
}

impl IntoIterator for TopicsList {
    type Item = Topic;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.topics.into_iter()
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
    use crate::configuration::domain::{Topic, TopicsList};
    use crate::error::Error::InvalidArgument;

    #[test]
    fn cannot_be_empty_from_vec_string() {
        let vec: Vec<String> = Vec::new();
        let sut = TopicsList::try_from(vec);

        assert_eq!(
            Err(InvalidArgument(
                "Topic collection could not be empty".to_string()
            )),
            sut
        );
    }

    #[test]
    fn cannot_be_empty_new_() {
        let sut = TopicsList::new(vec![]);

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

        assert_eq!(Err(InvalidArgument("Invalid topic name".to_string())), sut);
    }

    #[test]
    fn can_be_build_with_valid_topic_names_from_vec_string() {
        let topics = vec![
            String::from("contact.updated"),
            String::from("contact.created"),
        ];
        let sut = TopicsList::try_from(topics);

        assert!(sut.is_ok());
    }

    #[test]
    fn can_be_build_with_valid_topic_names_new() {
        let topics = vec![
            Topic::new("contact.updated").unwrap(),
            Topic::new("contact.created").unwrap(),
        ];
        let sut = TopicsList::new(topics);

        assert!(sut.is_ok());
    }

    #[test]
    fn can_iterate() {
        let a = Topic::new("contact.updated").unwrap();
        let b = Topic::new("contact.created").unwrap();
        let all = [a.clone(), b.clone()];

        let sut = TopicsList::new(vec![a, b]).unwrap();

        let mut count: u8 = 0;
        for topic in sut {
            assert!(all.contains(&topic));
            count += 1;
        }

        assert_eq!(2, count);
    }
}
