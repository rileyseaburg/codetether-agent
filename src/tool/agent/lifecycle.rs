//! Lifecycle identities, ownership, and terminal summaries.

use std::collections::HashMap;

use crate::session::Session;

use super::store;

const TASK_PREFIX: &str = "agent-session:";

pub(in crate::tool::agent) fn task_id(session: &Session) -> String {
    format!("{TASK_PREFIX}{}", session.id)
}

pub(in crate::tool::agent) fn task_to_agent(parent: Option<&str>) -> HashMap<String, String> {
    store::entries_for_parent(parent)
        .into_iter()
        .map(|entry| (task_id(&entry.session), entry.session.id))
        .collect()
}

pub(in crate::tool::agent) fn terminal_summary(tool_count: usize, error: Option<&str>) -> String {
    let activity = format!("{tool_count} tool call(s)");
    match error {
        Some(error) => format!(
            "{activity}; error: {}",
            crate::bus::payload::bounded(error, "… [truncated]")
        ),
        None => activity,
    }
}

pub(in crate::tool::agent) fn result_message(
    agent: &str,
    response: &str,
    error: Option<&str>,
) -> String {
    let status = if error.is_some() {
        "failed"
    } else {
        "completed"
    };
    let body = match (response.trim().is_empty(), error) {
        (false, Some(error)) => format!("{response}\n\nError: {error}"),
        (false, None) => response.to_string(),
        (true, Some(error)) => error.to_string(),
        (true, None) => "No response was produced".to_string(),
    };
    let body = crate::bus::payload::bounded(&body, "\n… [sub-agent result truncated]");
    format!("Sub-agent @{agent} {status}:\n{body}")
}

#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod tests;
