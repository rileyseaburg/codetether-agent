//! Hash snapshot bytes using the existing process-local hasher.
use std::hash::{Hash, Hasher};

/// Hash the serialized session content.
pub(super) fn hash_content(content: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}
