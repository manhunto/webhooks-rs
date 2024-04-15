use std::env;
use std::env::VarError;

pub trait EnvVar<T> {
    fn env(key: &str) -> T;

    fn env_or(key: &str, default: T) -> T;
}

pub struct Env {}

impl Env {
    fn get_env(key: &str) -> Result<String, VarError> {
        env::var(key)
    }
}

impl EnvVar<String> for Env {
    fn env(key: &str) -> String {
        Self::get_env(key).unwrap_or_else(|_| panic!("env {} is not configured", key))
    }

    fn env_or(key: &str, default: String) -> String {
        Self::get_env(key).unwrap_or(default)
    }
}

impl EnvVar<usize> for Env {
    fn env(key: &str) -> usize {
        <Self as EnvVar<String>>::env(key).parse().unwrap()
    }

    fn env_or(key: &str, default: usize) -> usize {
        match Self::get_env(key) {
            Ok(var) => var.parse().unwrap(),
            Err(_) => default,
        }
    }
}

impl EnvVar<u16> for Env {
    fn env(key: &str) -> u16 {
        <Self as EnvVar<usize>>::env(key) as u16
    }

    fn env_or(key: &str, default: u16) -> u16 {
        <Self as EnvVar<usize>>::env_or(key, default as usize) as u16
    }
}

#[cfg(test)]
mod tests {
    use temp_env::{with_var, with_var_unset};

    use crate::env::{Env, EnvVar};

    #[test]
    fn env_with_default_env_is_set() {
        with_var("TEST_STRING", Some("FOO"), || {
            assert_eq!("FOO", Env::env_or("TEST_STRING", "BAR".to_string()));
        });

        with_var("TEST_INT", Some("123"), || {
            let var: usize = Env::env_or("TEST_INT", 456);
            assert_eq!(123, var);
        });
    }

    #[test]
    fn env_with_default_env_is_not_set_then_pick_default() {
        assert_eq!("BAR", Env::env_or("TEST_STRING", "BAR".to_string()));

        let var: usize = Env::env_or("TEST_INT", 456);
        assert_eq!(456, var);
    }

    #[test]
    fn env_when_env_is_set() {
        with_var("TEST_STRING", Some("FOO"), || {
            let var: String = Env::env("TEST_STRING");

            assert_eq!(String::from("FOO"), var);
        });

        with_var("TEST_INT", Some("132"), || {
            let var: usize = Env::env("TEST_INT");

            assert_eq!(132, var);
        });
    }

    #[test]
    #[should_panic(expected = "env TEST_STRING is not configured")]
    fn env_without_string_env() {
        with_var_unset("TEST_STRING", || {
            let _: String = Env::env("TEST_STRING");
        })
    }

    #[test]
    #[should_panic(expected = "env TEST_INT is not configured")]
    fn env_without_int_env() {
        with_var_unset("TEST_INT", || {
            let _: usize = Env::env("TEST_INT");
        })
    }
}
