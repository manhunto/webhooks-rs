use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Serializer {}

impl Serializer {
    pub fn deserialize<T>(binary: &[u8]) -> T
        where T: DeserializeOwned
    {
        let msg = String::from_utf8_lossy(binary);

        serde_json::from_str(&msg).unwrap()
    }

    pub fn serialize<T>(value: T) -> Vec<u8> // is possible to return &[u8] ?
        where T: Serialize
    {
        let string = serde_json::to_string(&value);

        string.unwrap().as_bytes().to_vec()
    }
}