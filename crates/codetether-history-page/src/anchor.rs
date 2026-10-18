//! Stable fingerprints used to locate the visible history boundary.

use std::hash::{DefaultHasher, Hash, Hasher};

use codetether_provider_core::Message;

use super::Fingerprint;

const ANCHOR_MESSAGES: usize = 3;

/// Hash at most the first three messages.
///
/// # Arguments
/// * `messages` — Chronological message slice.
///
/// # Returns
/// Boundary fingerprints, possibly empty.
///
/// # Examples
/// ```
/// assert!(codetether_history_page::anchor::fingerprints(&[]).is_empty());
/// ```
pub fn fingerprints(messages: &[Message]) -> Vec<Fingerprint> {
    messages
        .iter()
        .take(ANCHOR_MESSAGES)
        .map(fingerprint)
        .collect()
}

/// Find the last matching boundary in the supplied history.
///
/// # Arguments
/// * `messages` — Chronological messages to search.
/// * `boundary` — Expected sequence of message fingerprints.
///
/// # Returns
/// Matching start index, or None for an absent or empty boundary.
///
/// # Examples
/// ```
/// assert_eq!(codetether_history_page::anchor::find(&[], &[]), None);
/// ```
pub fn find(messages: &[Message], boundary: &[Fingerprint]) -> Option<usize> {
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
