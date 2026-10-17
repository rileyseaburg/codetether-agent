//! Session-unique task IDs and live ownership mapping.
use crate::Host;
use std::collections::HashMap;

const TASK_PREFIX: &str = "agent-session:";

/// Format a child session ID as the existing lifecycle task ID.
/// # Arguments
/// * `session_id` — Durable child identity.
/// # Returns
/// The lifecycle task topic identity.
/// # Examples
/// ```rust
/// assert_eq!(codetether_agent_status::task_id("child"), "agent-session:child");
/// ```
pub fn task_id(session_id: &str) -> String {
    format!("{TASK_PREFIX}{}", session_id)
}

/// Map owned lifecycle task IDs to canonical child IDs.
/// # Arguments
/// * `parent` — Owner scope supplied to the live snapshot source.
/// # Returns
/// One task-to-child mapping per live snapshot.
/// # Examples
/// Use an application [Host] implementation for the live snapshot source.
pub fn task_to_agent<H: Host>(parent: Option<&str>) -> HashMap<String, String> {
    H::agents(parent)
        .into_iter()
        .map(|entry| (task_id(&entry.id), entry.id))
        .collect()
}
