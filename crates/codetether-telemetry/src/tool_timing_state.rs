//! Pending and completed tool timing state.
//! Shared timing state and operations are independent of application events.

use std::time::Instant;
/// Timing and outcome of current and last completed tool calls.
///
/// # Examples
/// ```
/// let state = codetether_telemetry::ToolTimingState::default();
/// assert!(state.last_tool_name.is_none());
/// ```
#[derive(Debug, Clone, Default)]
pub struct ToolTimingState {
    /// Name of the last completed tool.
    pub last_tool_name: Option<String>,
    /// Duration of the last completed tool.
    pub last_tool_latency_ms: Option<u64>,
    /// Outcome of the last completed tool.
    pub last_tool_success: Option<bool>,
    /// Name of the currently running tool.
    pub pending_tool_name: Option<String>,
    /// Start time of the currently running tool.
    pub pending_tool_started_at: Option<Instant>,
}
