use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;

pub fn env<T>(key: &str) -> T
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    get_env(key)
        .unwrap_or_else(|_| panic!("env {} is not configured", key))
        .parse()
        .unwrap()
}

pub fn env_with_default<T>(key: &str, default: T) -> T
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    if let Ok(var) = get_env(key) {
        return var.parse().unwrap();
    }

    default
}

fn get_env(key: &str) -> Result<String, VarError> {
    env::var(key)
}

#[cfg(test)]
mod tests {
    use temp_env::{with_var, with_var_unset};

    use crate::env::{env, env_with_default};

    #[test]
    fn env_with_default_env_is_set() {
        with_var("TEST_STRING", Some("FOO"), || {
            assert_eq!("FOO", env_with_default("TEST_STRING", "BAR".to_string()));
        });

        with_var("TEST_INT", Some("123"), || {
            let var: i32 = env_with_default("TEST_INT", 456);
            assert_eq!(123, var);
        });
    }

    #[test]
    fn env_with_default_env_is_not_set_then_pick_default() {
        assert_eq!("BAR", env_with_default("TEST_STRING", "BAR".to_string()));

        let var: i32 = env_with_default("TEST_INT", 456);
        assert_eq!(456, var);
    }

    #[test]
    fn env_when_env_is_set() {
        with_var("TEST_STRING", Some("FOO"), || {
            let var: String = env("TEST_STRING");

            assert_eq!(String::from("FOO"), var);
        });

        with_var("TEST_INT", Some("132"), || {
            let var: i32 = env("TEST_INT");

            assert_eq!(132, var);
        });
    }

    #[test]
    #[should_panic(expected = "env TEST_STRING is not configured")]
    fn env_without_string_env() {
        with_var_unset("TEST_STRING", || {
            let _: String = env("TEST_STRING");
        })
    }

    #[test]
    #[should_panic(expected = "env TEST_INT is not configured")]
    fn env_without_int_env() {
        with_var_unset("TEST_INT", || {
            let _: u32 = env("TEST_INT");
        })
    }
}
