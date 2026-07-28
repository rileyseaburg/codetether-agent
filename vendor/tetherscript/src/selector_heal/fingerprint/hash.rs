//! Stable hashing helpers for selector fingerprints.

use super::node::DomNode;
use std::hash::{Hash, Hasher};

pub fn norm(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn stable_attrs(n: &DomNode) -> String {
    ["data-testid", "aria-label", "role", "name", "type", "href"]
        .iter()
        .filter_map(|k| n.attr(k).map(|v| format!("{k}={v}")))
        .collect::<Vec<_>>()
        .join("|")
}

pub fn hash_str<T: Hash>(v: &T) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    v.hash(&mut h);
    h.finish()
}
