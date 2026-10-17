//! Bounded lifecycle summaries and parent-facing result text.

/// Describe tool activity and retain bounded terminal error evidence.
/// # Arguments
/// * `tool_count` — Number of completed tool calls.
/// * `error` — Optional terminal error text.
/// # Returns
/// A bounded lifecycle summary for the bus.
/// # Examples
/// ```rust
/// assert_eq!(codetether_agent_status::terminal_summary(2, None), "2 tool call(s)");
/// ```
pub fn terminal_summary(tool_count: usize, error: Option<&str>) -> String {
    let activity = format!("{tool_count} tool call(s)");
    match error {
        Some(error) => format!(
            "{activity}; error: {}",
            codetether_bus_core::payload::bounded(error, "… [truncated]")
        ),
        None => activity,
    }
}

/// Format the existing bounded child result delivered to its parent.
/// # Arguments
/// * `agent` — Child display name.
/// * `response` — Completed child response text.
/// * `error` — Optional terminal error text.
/// # Returns
/// A completion/failure message retaining available result evidence.
/// # Examples
/// ```rust
/// assert!(codetether_agent_status::result_message("child", "done", None).contains("done"));
/// ```
pub fn result_message(agent: &str, response: &str, error: Option<&str>) -> String {
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
    let body = codetether_bus_core::payload::bounded(&body, "\n… [sub-agent result truncated]");
    format!("Sub-agent @{agent} {status}:\n{body}")
}
#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod tests;
