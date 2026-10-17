//! Exact and prefix wildcard topic filtering.
use super::*;

/// Wildcard topic matching (supports `*` and `prefix.*` patterns).
pub(super) fn bus_topic_matches(topic: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return topic.starts_with(prefix);
    }
    topic == pattern
}
