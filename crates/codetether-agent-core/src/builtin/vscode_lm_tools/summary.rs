//! Condensed summaries of VS Code `languageModelTools` manifest entries.

use serde_json::Value;

use super::text::compact;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct LmToolSummary {
    pub name: String,
    pub reference: Option<String>,
    pub description: String,
    pub required: Vec<String>,
}

pub(super) fn summaries(root: &Value) -> Vec<LmToolSummary> {
    let Some(items) = root
        .pointer("/contributes/languageModelTools")
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };
    items.iter().map(item_summary).collect()
}

fn item_summary(item: &Value) -> LmToolSummary {
    LmToolSummary {
        name: field(item, "name"),
        reference: optional_field(item, "toolReferenceName"),
        description: description(item),
        required: required_fields(item),
    }
}

fn description(item: &Value) -> String {
    ["userDescription", "displayName", "modelDescription"]
        .iter()
        .find_map(|key| optional_field(item, key))
        .map(|value| compact(&value))
        .unwrap_or_default()
}

fn required_fields(item: &Value) -> Vec<String> {
    item.pointer("/inputSchema/required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn field(item: &Value, key: &str) -> String {
    optional_field(item, key).unwrap_or_default()
}

fn optional_field(item: &Value, key: &str) -> Option<String> {
    item.get(key).and_then(Value::as_str).map(str::to_string)
}
