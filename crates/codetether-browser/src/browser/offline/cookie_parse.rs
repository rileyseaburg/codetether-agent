//! Minimal Set-Cookie parser. Captures name, value, and lowercase attribute map.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CookieRecord {
    pub name: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attrs: BTreeMap<String, String>,
}

pub fn parse(header: &str) -> Option<CookieRecord> {
    let mut parts = header.split(';');
    let (name, value) = split_pair(parts.next()?)?;
    let mut attrs = BTreeMap::new();
    for raw in parts {
        if let Some((k, v)) = split_pair(raw) {
            attrs.insert(k.to_ascii_lowercase(), v);
        } else {
            attrs.insert(raw.trim().to_ascii_lowercase(), String::new());
        }
    }
    Some(CookieRecord { name, value, attrs })
}

fn split_pair(raw: &str) -> Option<(String, String)> {
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    match raw.split_once('=') {
        Some((k, v)) => Some((k.trim().to_string(), v.trim().to_string())),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_session_cookie() {
        let c = parse("session=abc; Path=/; HttpOnly; Secure").unwrap();
        assert_eq!(c.name, "session");
        assert_eq!(c.value, "abc");
        assert_eq!(c.attrs.get("path").unwrap(), "/");
        assert!(c.attrs.contains_key("httponly"));
    }
}
