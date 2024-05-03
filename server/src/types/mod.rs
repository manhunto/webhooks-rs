macro_rules! make_ksuid {
    ($name: ident, $prefix: literal) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        pub struct $name {
            id: svix_ksuid::Ksuid,
        }

        impl $name {
            const PREFIX: &'static str = $prefix;
            const TERMINATOR: char = '_';

            pub fn new() -> Self {
                use svix_ksuid::{Ksuid, KsuidLike};

                Self {
                    id: Ksuid::new(None, None),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}{}", Self::PREFIX, Self::TERMINATOR, self.id)
            }
        }

        impl TryFrom<String> for $name {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                use std::str::FromStr;

                Self::from_str(value.as_str())
            }
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use itertools::Itertools;

                let (prefix, id) = s
                    .split_terminator(Self::TERMINATOR)
                    .collect_tuple()
                    .unwrap();

                if prefix != Self::PREFIX {
                    return Err(format!(
                        "'{}' type should have prefix '{}' but have '{}'",
                        stringify!($name),
                        Self::PREFIX,
                        prefix,
                    ));
                }

                Ok(Self {
                    id: svix_ksuid::Ksuid::from_str(id).unwrap(),
                })
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

make_ksuid!(MessageId, "msg");
make_ksuid!(RoutedMessageId, "rmsg");
make_ksuid!(ApplicationId, "app");
make_ksuid!(EndpointId, "ep");

#[cfg(test)]
mod ksuid_tests {
    use std::str::FromStr;

    use itertools::Itertools;
    use svix_ksuid::{Ksuid, KsuidLike};

    make_ksuid!(TestId, "test");

    #[test]
    fn can_be_build_from_string() {
        assert!(TestId::from_str("test_1srOrx2ZWZBpBUvZwXKQmoEYga2").is_ok())
    }

    #[test]
    fn cannot_build_from_invalid_prefix() {
        let ksuid = Ksuid::new(None, None);
        let id = format!("invalid_{}", ksuid);

        assert_eq!(
            Err("'TestId' type should have prefix 'test' but have 'invalid'".to_string()),
            TestId::try_from(id)
        );
    }

    #[test]
    fn eq_test() {
        let a = TestId::from_str("test_1srOrx2ZWZBpBUvZwXKQmoEYga2").unwrap();
        let b = TestId::from_str("test_1srOrx2ZWZBpBUvZwXKQmoEYga2").unwrap();
        let c = TestId::from_str("test_0ujtsYcgvSTl8PAuAdqWYSMnLOv").unwrap();

        assert!(a.eq(&b));
        assert!(a.eq(&a));
        assert!(!a.eq(&c));
        assert!(!b.eq(&c));
    }

    #[test]
    fn display_with_prefix() {
        let sut = TestId::new();

        let binding = sut.to_string();
        let (prefix, _) = binding.split_terminator('_').collect_tuple().unwrap();

        assert_eq!("test", prefix);
    }
}
