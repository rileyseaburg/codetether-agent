//! parse value metadata parsing.
use super::*;

pub(super) fn parse_value(value: &serde_json::Value) -> Option<Vec<String>> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .filter(|items| !items.is_empty())
        .or_else(|| {
            value
                .as_str()
                .map(|items| {
                    items
                        .split(['\n', ','])
                        .map(str::trim)
                        .filter(|item| !item.is_empty())
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                })
                .filter(|items| !items.is_empty())
        })
}
