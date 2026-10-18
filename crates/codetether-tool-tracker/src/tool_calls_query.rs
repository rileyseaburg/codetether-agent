//! Query helpers over [`ToolCallTracker`](super::tool_calls::ToolCallTracker).
//!
//! Built on [`ToolCallTracker::open_calls`] so the in-flight map stays private.
//! These answer "what is agent X doing right now" and drive watchdog alerts
//! from bus-observed state.

use std::time::{Duration, Instant};

use super::tool_calls::{OpenToolCall, ToolCallTracker};

impl ToolCallTracker {
    /// `true` while any open call is still inside the runtime it asked for
    /// (`timeout` / `timeout_secs`). Such a call is working, not stalled.
    ///
    /// # Returns
    /// Whether any open call remains inside its declared timeout.
    ///
    /// # Examples
    /// ```
    /// let tracker = codetether_tool_tracker::ToolCallTracker::default();
    /// assert!(!tracker.within_declared_budget());
    /// ```
    pub fn within_declared_budget(&self) -> bool {
        let now = Instant::now();
        self.open_calls().any(|c| {
            c.declared_timeout
                .is_some_and(|budget| now.duration_since(c.started_at) < budget)
        })
    }

    /// The oldest open call that has outlived both the watchdog `timeout` and
    /// its own declared budget. A `bash` asked to run for 600 s is not stalled
    /// at 240 s.
    ///
    /// # Arguments
    /// * `timeout` — Watchdog inactivity budget.
    ///
    /// # Returns
    /// The oldest call past both budgets, if one exists.
    ///
    /// # Examples
    /// ```
    /// let tracker = codetether_tool_tracker::ToolCallTracker::default();
    /// assert!(tracker.stalled(std::time::Duration::ZERO).is_none());
    /// ```
    pub fn stalled(&self, timeout: Duration) -> Option<&OpenToolCall> {
        let now = Instant::now();
        self.open_calls()
            .filter(|c| {
                let budget = timeout.max(c.declared_timeout.unwrap_or(Duration::ZERO));
                now.duration_since(c.started_at) >= budget
            })
            .min_by_key(|c| c.started_at)
    }

    /// Open calls issued by a specific agent (e.g. a named sub-agent).
    ///
    /// # Arguments
    /// * `agent_id` — Agent name to filter by.
    ///
    /// # Returns
    /// An iterator over the named agent's current calls.
    ///
    /// # Examples
    /// ```
    /// let tracker = codetether_tool_tracker::ToolCallTracker::default();
    /// assert_eq!(tracker.for_agent("agent").count(), 0);
    /// ```
    pub fn for_agent<'a>(
        &'a self,
        agent_id: &'a str,
    ) -> impl Iterator<Item = &'a OpenToolCall> + 'a {
        self.open_calls().filter(move |c| c.agent_id == agent_id)
    }

    /// The tool name a specific agent is currently running, if any.
    ///
    /// # Arguments
    /// * `agent_id` — Agent name to query.
    ///
    /// # Returns
    /// The oldest open tool name for the agent, if present.
    ///
    /// # Examples
    /// ```
    /// let tracker = codetether_tool_tracker::ToolCallTracker::default();
    /// assert_eq!(tracker.current_tool("agent"), None);
    /// ```
    pub fn current_tool(&self, agent_id: &str) -> Option<&str> {
        self.open_calls()
            .filter(|c| c.agent_id == agent_id)
            .min_by_key(|c| c.started_at)
            .map(|c| c.tool_name.as_str())
    }
}
