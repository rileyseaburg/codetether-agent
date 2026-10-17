//! prepare_relay_handoff formatting.
/// Render prepare_relay_handoff output.
///
/// # Arguments
/// * `from_agent` - Previous agent name.
/// * `output` - Previous response.
/// * `original_task` - Task retained across handoffs.
/// # Returns
/// Handoff text with the original task and truncated response.
/// # Examples
/// ```
/// let text = codetether_ralph_core::presentation::prepare_relay_handoff("planner", "done", "task");
/// assert!(text.contains("@planner"));
/// ```

/// Format relay handoff text between agents.
pub fn prepare_relay_handoff(from_agent: &str, output: &str, original_task: &str) -> String {
    let max_len = 6_000;
    let relay_payload = if output.len() > max_len {
        let truncated: String = output.chars().take(max_len).collect();
        format!("{truncated}\n... (truncated)")
    } else {
        output.to_string()
    };

    format!(
        "Original task:\n{original_task}\n\n\
             Incoming handoff from @{from_agent}:\n{relay_payload}\n\n\
             Continue the work from this handoff. Keep your response focused and provide concrete next steps."
    )
}
