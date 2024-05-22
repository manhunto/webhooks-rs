macro_rules! make_ksuid {
    ($name: ident, $prefix: literal) => {
        #[derive(Clone, Copy, Eq, PartialEq)]
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

            pub fn to_base62(self) -> String {
                use svix_ksuid::KsuidLike;

                self.id.to_base62()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}{}", Self::PREFIX, Self::TERMINATOR, self.id)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        impl TryFrom<String> for $name {
            type Error = crate::error::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                use std::str::FromStr;

                Self::from_str(value.as_str())
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use itertools::Itertools;

                let tuple = s
                    .split_terminator(Self::TERMINATOR)
                    .collect_tuple();

                if tuple.is_none() {
                    return Err(crate::error::Error::InvalidArgument(format!(
                        "'{}' type should has '{}' prefix and valid id. Example '{}_1srOrx2ZWZBpBUvZwXKQmoEYga2'",
                        stringify!($name),
                        Self::PREFIX,
                        Self::PREFIX,
                    )));
                }

                let (prefix, id) = tuple.unwrap();

                if prefix != Self::PREFIX {
                    return Err(crate::error::Error::InvalidArgument(format!(
                        "'{}' type should have prefix '{}' but have '{}'",
                        stringify!($name),
                        Self::PREFIX,
                        prefix,
                    )));
                }

                let ksuid = svix_ksuid::Ksuid::from_str(id);
                if ksuid.is_err() {
                    return Err(crate::error::Error::InvalidArgument(format!(
                        "'{}' type received invalid id '{}'",
                        stringify!($name),
                        id,
                    )));
                }

                Ok(Self {
                    id: ksuid.unwrap(),
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

make_ksuid!(EventId, "evt");
make_ksuid!(RoutedMessageId, "rmsg");
make_ksuid!(ApplicationId, "app");
make_ksuid!(EndpointId, "ep");

#[cfg(test)]
mod ksuid_tests {
    use std::str::FromStr;

    use itertools::Itertools;
    use test_case::test_case;

    use crate::error::Error::InvalidArgument;

    make_ksuid!(TestId, "test");

    #[test]
    fn can_be_build_from_string() {
        assert!(TestId::from_str("test_1srOrx2ZWZBpBUvZwXKQmoEYga2").is_ok())
    }

    #[test_case("invalid_1srOrx2ZWZBpBUvZwXKQmoEYga2", "'TestId' type should have prefix 'test' but have 'invalid'"; "invalid prefix")]
    #[test_case("1srOrx2ZWZBpBUvZwXKQmoEYga2", "'TestId' type should has 'test' prefix and valid id. Example 'test_1srOrx2ZWZBpBUvZwXKQmoEYga2'"; "without prefix")]
    #[test_case("invalid_", "'TestId' type should has 'test' prefix and valid id. Example 'test_1srOrx2ZWZBpBUvZwXKQmoEYga2'"; "only prefix")]
    #[test_case("test_foo", "'TestId' type received invalid id 'foo'"; "invalid id")]
    fn invalid(id: &str, error: &str) {
        assert_eq!(
            Err(InvalidArgument(error.to_string())),
            TestId::try_from(id.to_string())
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

    #[test]
    fn debug_format() {
        let sut = TestId::from_str("test_1srOrx2ZWZBpBUvZwXKQmoEYga2").unwrap();

        assert_eq!("test_1srOrx2ZWZBpBUvZwXKQmoEYga2", &format!("{:?}", sut));
    }

    #[test]
    fn test_to_base62() {
        let sut = TestId::from_str("test_1srOrx2ZWZBpBUvZwXKQmoEYga2").unwrap();

        assert_eq!("1srOrx2ZWZBpBUvZwXKQmoEYga2", sut.to_base62());
    }
}
