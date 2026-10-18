//! Bounded mutation of the open tool-call tracker.

use std::time::Instant;

use codetether_bus_core::BusMessage;

use super::{OpenToolCall, ToolCallTracker};

#[path = "tool_calls_timeout.rs"]
mod timeout;
use timeout::declared_timeout;

pub(crate) const MAX_OPEN_CALLS: usize = 256;

impl ToolCallTracker {
    /// Update tracking from a bus message. No-op for unrelated kinds.
    ///
    /// # Arguments
    /// * `message` — Bus request, response or unrelated event.
    ///
    /// # Returns
    /// Updates open calls in place, retaining at most 256 distinct requests.
    ///
    /// # Examples
    /// ```
    /// let mut tracker = codetether_tool_tracker::ToolCallTracker::default();
    /// let request = codetether_bus_core::BusMessage::ToolRequest { request_id: "r".into(), agent_id: "a".into(), tool_name: "read".into(), arguments: serde_json::Value::Null, step: 1 };
    /// tracker.observe(&request);
    /// assert_eq!(tracker.open_calls().count(), 1);
    /// ```
    pub fn observe(&mut self, message: &BusMessage) {
        match message {
            BusMessage::ToolRequest {
                request_id,
                agent_id,
                tool_name,
                step,
                arguments,
            } => self.insert(
                request_id,
                OpenToolCall {
                    agent_id: agent_id.clone(),
                    tool_name: tool_name.clone(),
                    step: *step,
                    started_at: Instant::now(),
                    declared_timeout: declared_timeout(arguments),
                },
            ),
            BusMessage::ToolResponse { request_id, .. } => {
                self.open.remove(request_id);
            }
            _ => {}
        }
    }
}
