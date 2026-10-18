//! Human-readable watchdog stall descriptions.
use codetether_tool_tracker::OpenToolCall;
pub(crate) fn tool_stall(call: &OpenToolCall, secs: u64) -> String {
    format!(
        "⚠ {} tool '{}' (step {}) open {secs}s with no response.",
        call.agent_id, call.tool_name, call.step
    )
}
pub(crate) fn request_stall(no_first_token: bool, secs: u64) -> String {
    if no_first_token {
        format!("⚠ No response after {secs}s. Provider may be unreachable.")
    } else {
        format!("⚠ No events for {secs}s. Stalled request detected.")
    }
}
