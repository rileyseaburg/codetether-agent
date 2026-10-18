//! Device authorization interval responsibilities.
use serde::{Deserialize, de, de::Deserializer};

pub(super) fn deserialize_interval_seconds<'de, D>(
    deserializer: D,
) -> std::result::Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IntervalValue {
        Number(u64),
        String(String),
    }

    let value = Option::<IntervalValue>::deserialize(deserializer)?;
    match value {
        Some(IntervalValue::Number(value)) => Ok(value),
        Some(IntervalValue::String(value)) => value
            .trim()
            .parse::<u64>()
            .map_err(|error| de::Error::custom(format!("invalid interval value: {error}"))),
        None => Ok(0),
    }
}
