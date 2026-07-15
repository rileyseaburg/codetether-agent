//! Stable fingerprints used to locate the visible history boundary.

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::provider::Message;

use super::Fingerprint;

const ANCHOR_MESSAGES: usize = 3;

pub(super) fn fingerprints(messages: &[Message]) -> Vec<Fingerprint> {
    messages
        .iter()
        .take(ANCHOR_MESSAGES)
        .map(fingerprint)
        .collect()
}

pub(super) fn find(messages: &[Message], boundary: &[Fingerprint]) -> Option<usize> {
    if boundary.is_empty() {
        return None;
    }
    messages
        .windows(boundary.len())
        .rposition(|window| fingerprints(window) == boundary)
}

fn fingerprint(message: &Message) -> Fingerprint {
    let mut hasher = DefaultHasher::new();
    serde_json::to_vec(message)
        .unwrap_or_default()
        .hash(&mut hasher);
    hasher.finish()
}
