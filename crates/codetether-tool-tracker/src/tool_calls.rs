//! Bus-driven tool-call tracking for watchdog TUI alerts.
//!
//! Observes [`codetether_bus_core::BusMessage::ToolRequest`] / [`codetether_bus_core::BusMessage::ToolResponse`] and
//! keeps the set of *in-flight* tool calls keyed by `request_id`. A request
//! with no matching response is an open call; the watchdog uses
//! [`ToolCallTracker::stalled`] to alert when one stays open too long.

use std::collections::HashMap;
use std::time::Instant;

#[path = "tool_calls_observe.rs"]
pub(super) mod observe;

/// One in-flight tool call observed on the bus, attributed to its agent.
///
/// # Examples
/// ```
/// let call = codetether_tool_tracker::OpenToolCall { agent_id: "agent".into(), tool_name: "read".into(), step: 1, started_at: std::time::Instant::now(), declared_timeout: None };
/// assert_eq!(call.step, 1);
/// ```
#[derive(Debug, Clone)]
pub struct OpenToolCall {
    /// Agent that issued the call (`ToolRequest.agent_id`).
    pub agent_id: String,
    /// Tool (function) name (`ToolRequest.tool_name`).
    pub tool_name: String,
    /// Agent-loop step the call belongs to.
    pub step: usize,
    /// When the request was observed locally.
    pub started_at: Instant,
    /// Runtime the caller asked for (`timeout` / `timeout_secs` argument).
    /// A call is not stalled until it has outlived this budget too.
    pub declared_timeout: Option<std::time::Duration>,
}

/// Tracks open tool calls keyed by `request_id`.
///
/// # Examples
/// ```
/// let tracker = codetether_tool_tracker::ToolCallTracker::default();
/// assert_eq!(tracker.open_calls().count(), 0);
/// ```
#[derive(Debug, Default)]
pub struct ToolCallTracker {
    open: HashMap<String, OpenToolCall>,
}

impl ToolCallTracker {
    /// Iterator over every open call (used by the per-agent query helpers).
    ///
    /// # Examples
    /// ```
    /// let tracker = codetether_tool_tracker::ToolCallTracker::default();
    /// assert!(tracker.open_calls().next().is_none());
    /// ```
    pub fn open_calls(&self) -> impl Iterator<Item = &OpenToolCall> {
        self.open.values()
    }
}

#[path = "tool_calls_insert.rs"]
mod insert;
