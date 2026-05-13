//! Stable cache keys for background RLM.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(super) fn key(tool: &str, input: &serde_json::Value, content: &str) -> u64 {
    let mut h = DefaultHasher::new();
    "rlm-background-v1".hash(&mut h);
    tool.hash(&mut h);
    input.to_string().hash(&mut h);
    content.hash(&mut h);
    h.finish()
}
