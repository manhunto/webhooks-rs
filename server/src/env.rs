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
    get_env(key).map_err(|_| default).unwrap().parse().unwrap()
}

fn get_env(key: &str) -> Result<String, VarError> {
    env::var(key)
}
