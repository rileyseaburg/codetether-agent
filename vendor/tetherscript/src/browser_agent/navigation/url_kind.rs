//! URL comparison helpers for navigation classification.

use super::result::NavigationKind;

pub(crate) fn transition(from: &str, to: &str) -> NavigationKind {
    if strip_hash(from) == strip_hash(to) && from != to {
        NavigationKind::SameDocument
    } else {
        NavigationKind::DocumentReplacement
    }
}

pub(crate) fn same_document_hash<'a>(current: &str, next: &'a str) -> Option<&'a str> {
    let (next_base, hash) = next.split_once('#')?;
    if strip_hash(current) == next_base {
        Some(hash)
    } else {
        None
    }
}

fn strip_hash(value: &str) -> &str {
    value.split('#').next().unwrap_or(value)
}
