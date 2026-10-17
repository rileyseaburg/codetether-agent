use anyhow::Result;
use serde::de::DeserializeOwned;

pub(super) fn parse_string<T>(value: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    Ok(serde_json::from_value(serde_json::Value::String(
        value.to_owned(),
    ))?)
}
