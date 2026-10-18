//! Transcript-prefix fingerprints used to reject stale prepared context.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use codetether_provider_core::Message;

/// Fingerprint serialized messages using the original hashing scheme.
/// # Arguments
/// * `messages` — Transcript prefix to fingerprint.
/// # Returns
/// The stable-for-this-build hash of its JSON representation.
pub fn fingerprint(messages: &[Message]) -> u64 {
    let mut hasher = DefaultHasher::new();
    serde_json::to_string(messages)
        .unwrap_or_default()
        .hash(&mut hasher);
    hasher.finish()
}
