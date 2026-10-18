//! Shared validity checks for prepared transcript prefixes.

use codetether_provider_core::Message;

use crate::fingerprint;

/// Check whether a prepared prefix still matches an extended transcript.
/// # Arguments
/// * `messages` — Current transcript.
/// * `count` — Number of messages originally fingerprinted.
/// * `expected` — Fingerprint recorded when preparation finished.
/// # Returns
/// False when the prefix no longer exists or has changed.
pub fn matches(messages: &[Message], count: usize, expected: u64) -> bool {
    messages
        .get(..count)
        .map(fingerprint)
        .is_some_and(|value| value == expected)
}
