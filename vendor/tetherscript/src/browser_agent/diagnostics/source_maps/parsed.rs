//! Minimal source-map JSON extraction.

use crate::json;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct ParsedSourceMap {
    pub sources: Vec<String>,
    pub mappings: String,
    pub source_root: String,
}

pub fn parse(input: &str) -> Option<ParsedSourceMap> {
    let Value::Map(map) = json::parse_str(input).ok()? else {
        return None;
    };
    let map = map.borrow();
    Some(ParsedSourceMap {
        sources: string_list(map.get("sources")?)?,
        mappings: string_field(map.get("mappings")?)?,
        source_root: map
            .get("sourceRoot")
            .and_then(string_field)
            .unwrap_or_default(),
    })
}

fn string_field(value: &Value) -> Option<String> {
    match value {
        Value::Str(text) => Some((**text).clone()),
        _ => None,
    }
}

fn string_list(value: &Value) -> Option<Vec<String>> {
    let Value::List(items) = value else {
        return None;
    };
    items.borrow().iter().map(string_field).collect()
}
