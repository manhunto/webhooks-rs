use crate::application::domain::ApplicationId;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use svix_ksuid::{Ksuid, KsuidLike};
use url::Url;

#[derive(Debug, Clone, derive::Ksuid)]
#[prefix = "ep"]
pub struct EndpointId {
    id: String,
}

impl TryFrom<String> for EndpointId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (prefix, _) = value.split_terminator('_').collect_tuple().unwrap();

        if prefix != "ep" {
            return Err(format!(
                "{} should have prefix {} but have {}",
                "Endpoint", "ep", prefix,
            ));
        }

        Ok(EndpointId { id: value })
    }
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
