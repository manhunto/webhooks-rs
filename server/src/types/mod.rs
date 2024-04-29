macro_rules! make_ksuid {
    ($name: ident, $prefix: literal) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub struct $name {
            id: svix_ksuid::Ksuid,
        }

        impl $name {
            const PREFIX: &'static str = $prefix;

            pub fn new() -> Self {
                use svix_ksuid::{Ksuid, KsuidLike};

                Self {
                    id: Ksuid::new(None, None),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}_{}", Self::PREFIX, self.id)
            }
        }

        impl TryFrom<String> for $name {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                use itertools::Itertools;
                use std::str::FromStr;

                let (prefix, id) = value.split_terminator('_').collect_tuple().unwrap();

                if prefix != Self::PREFIX {
                    return Err(format!(
                        "{} should have prefix {} but have {}",
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
