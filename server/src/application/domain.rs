use itertools::Itertools;
use std::fmt::{Display, Formatter};
use svix_ksuid::{Ksuid, KsuidLike};

#[derive(Debug, Clone, derive::Ksuid)]
#[prefix = "app"]
pub struct ApplicationId {
    id: String,
}

impl TryFrom<String> for ApplicationId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (prefix, _) = value.split_terminator('_').collect_tuple().unwrap();

        if prefix != "app" {
            return Err(format!(
                "{} should have prefix {} but have {}",
                "ApplicationId", "app", prefix,
            ));
        }

        Ok(ApplicationId { id: value })
    }
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
