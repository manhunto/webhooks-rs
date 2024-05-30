use chrono::{DateTime, Utc};

pub enum Clock {
    Chrono,
    #[cfg(test)]
    Fixed(DateTime<Utc>),
}

impl Clock {
    #[must_use]
    pub fn now(&self) -> DateTime<Utc> {
        match self {
            Clock::Chrono => Utc::now(),
            #[cfg(test)]
            Clock::Fixed(date) => *date,
        }
    }

    #[must_use]
    pub fn chrono() -> Self {
        Self::Chrono
    }

    #[cfg(test)]
    pub fn fixed(time: DateTime<Utc>) -> Self {
        Self::Fixed(time)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::time::Clock;

    #[test]
    fn test_fixed_time() {
        let date = Utc.with_ymd_and_hms(2024, 5, 30, 14, 0, 0).unwrap();

        let sut = Clock::fixed(date);

        assert_eq!(date, sut.now());
    }

    #[test]
    // fixme: is it safe to test it in that way?
    fn test_chrono_time() {
        let sut = Clock::chrono();
        let expected_now = Utc::now();
        let now = sut.now();

        let delta = now - expected_now;

        assert_eq!(0, delta.num_milliseconds());
    }
}
