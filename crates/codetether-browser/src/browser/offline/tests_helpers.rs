//! Shared test helpers for the offline browser primitives.

#![cfg(test)]

use std::collections::BTreeMap;

use super::cookie_parse::CookieRecord;

pub(crate) fn rec(name: &str, value: &str, path: &str) -> CookieRecord {
    rec_with(name, value, path, None)
}

pub(crate) fn rec_with(name: &str, value: &str, path: &str, domain: Option<&str>) -> CookieRecord {
    let mut attrs = BTreeMap::new();
    attrs.insert("path".into(), path.into());
    if let Some(d) = domain {
        attrs.insert("domain".into(), d.into());
    }
    CookieRecord {
        name: name.into(),
        value: value.into(),
        attrs,
    }
}
