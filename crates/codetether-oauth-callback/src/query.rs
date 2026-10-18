//! OAuth callback query handling.
use std::collections::HashMap;

pub(super) fn parse_query_pairs(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    for pair in query.split('&') {
        if pair.trim().is_empty() {
            continue;
        }

        let (raw_key, raw_value) = match pair.split_once('=') {
            Some((key, value)) => (key, value),
            None => (pair, ""),
        };
        let key = decode_query_component(raw_key);
        let value = decode_query_component(raw_value);
        params.insert(key, value);
    }

    params
}

pub(super) fn decode_query_component(component: &str) -> String {
    match urlencoding::decode(component) {
        Ok(value) => value.into_owned(),
        Err(_) => component.to_string(),
    }
}
