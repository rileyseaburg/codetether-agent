//! Match exact, prefix and suffix bus topic filters.
use super::*;

// ─── Helpers ─────────────────────────────────────────────────────────────

/// Topic pattern matching (mirrors `server/mod.rs::topic_matches`).
pub(super) fn topic_matches(topic: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return topic.starts_with(prefix);
    }
    if let Some(suffix) = pattern.strip_prefix(".*") {
        return topic.ends_with(suffix);
    }
    topic == pattern
}
