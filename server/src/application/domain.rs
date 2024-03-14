use itertools::Itertools;
use std::fmt::{Display, Formatter};
use svix_ksuid::{Ksuid, KsuidLike};

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
