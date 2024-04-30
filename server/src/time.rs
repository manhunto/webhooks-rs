use chrono::{DateTime, Utc};

pub enum Clock {
    Chrono,
    Fixed(DateTime<Utc>),
}

impl Clock {
    pub fn now(&self) -> DateTime<Utc> {
        match self {
            Clock::Chrono => Utc::now(),
            Clock::Fixed(date) => *date,
        }
    }

    pub fn chrono() -> Self {
        Self::Chrono
    }
}
