use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

pub fn fixtures() -> Value {
    serde_json::from_str(include_str!("../../fixtures/wire.json")).unwrap()
}

pub fn round_trip<T: DeserializeOwned + Serialize>(value: &Value) {
    let decoded: T = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), *value);
}
