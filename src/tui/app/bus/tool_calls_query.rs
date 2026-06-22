//! Query helpers over [`ToolCallTracker`](super::tool_calls::ToolCallTracker).
//!
//! Built on [`ToolCallTracker::open_calls`] so the in-flight map stays private.
//! These answer "what is agent X doing right now" and drive watchdog alerts
//! from bus-observed state.

use std::time::{Duration, Instant};

use super::tool_calls::{OpenToolCall, ToolCallTracker};

impl ToolCallTracker {
    /// Oldest open call exceeding `timeout`, if any (watchdog stall alert).
    pub fn stalled(&self, timeout: Duration) -> Option<&OpenToolCall> {
        let now = Instant::now();
        self.open_calls()
            .filter(|c| now.duration_since(c.started_at) >= timeout)
            .min_by_key(|c| c.started_at)
    }

    /// Open calls issued by a specific agent (e.g. a named sub-agent).
    pub fn for_agent<'a>(
        &'a self,
        agent_id: &'a str,
    ) -> impl Iterator<Item = &'a OpenToolCall> + 'a {
        self.open_calls().filter(move |c| c.agent_id == agent_id)
    }

    /// The tool name a specific agent is currently running, if any.
    pub fn current_tool(&self, agent_id: &str) -> Option<&str> {
        self.open_calls()
            .filter(|c| c.agent_id == agent_id)
            .min_by_key(|c| c.started_at)
            .map(|c| c.tool_name.as_str())
    }
}
