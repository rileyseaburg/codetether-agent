//! Transcript-prefix fingerprints used to reject stale prepared context.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::provider::Message;

pub(super) fn messages(messages: &[Message]) -> u64 {
    let mut hasher = DefaultHasher::new();
    serde_json::to_string(messages)
        .unwrap_or_default()
        .hash(&mut hasher);
    hasher.finish()
}
