//! Indexing helpers for cookie_diff: composite (name, domain, path) key + duplicate detection.

use std::collections::HashMap;

use super::cookie_parse::CookieRecord;

pub(super) fn index(jar: &[CookieRecord]) -> (HashMap<String, &CookieRecord>, Vec<String>) {
    let mut map = HashMap::new();
    let mut dups = Vec::new();
    for c in jar {
        let key = composite_key(c);
        if map.insert(key.clone(), c).is_some() {
            dups.push(key);
        }
    }
    (map, dups)
}

pub(super) fn composite_key(c: &CookieRecord) -> String {
    let dom = c.attrs.get("domain").cloned().unwrap_or_default();
    let path = c.attrs.get("path").cloned().unwrap_or_default();
    format!("{}|{}|{}", c.name, dom, path)
}
