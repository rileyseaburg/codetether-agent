//! Existing topic-claim wildcard matching semantics.

/// Check if a topic matches a pattern.
/// Supports wildcards: `agent.*` matches `agent.123`, `agent.*.events` matches `agent.456.events`
/// # Arguments
/// * `topic` - Actual envelope topic.
/// * `pattern` - Exact topic or supported prefix/suffix wildcard.
/// # Returns
/// Whether the existing topic-claim matching rules permit the topic.
/// # Examples
/// ```
/// assert!(codetether_bus_sse::topic_matches("agent.1", "agent.*"));
/// ```
pub fn topic_matches(topic: &str, pattern: &str) -> bool {
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
