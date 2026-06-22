//! Bus-driven tool-call tracking for watchdog TUI alerts.
//!
//! Observes [`BusMessage::ToolRequest`] / [`BusMessage::ToolResponse`] and
//! keeps the set of *in-flight* tool calls keyed by `request_id`. A request
//! with no matching response is an open call; the watchdog uses
//! [`ToolCallTracker::stalled`] to alert when one stays open too long.

use std::collections::HashMap;
use std::time::Instant;

use crate::bus::BusMessage;

/// One in-flight tool call observed on the bus, attributed to its agent.
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
}

/// Tracks open tool calls keyed by `request_id`.
#[derive(Debug, Default)]
pub struct ToolCallTracker {
    open: HashMap<String, OpenToolCall>,
}

impl ToolCallTracker {
    /// Update tracking from a bus message. No-op for unrelated kinds.
    pub fn observe(&mut self, message: &BusMessage) {
        match message {
            BusMessage::ToolRequest {
                request_id,
                agent_id,
                tool_name,
                step,
                ..
            } => {
                self.open.insert(
                    request_id.clone(),
                    OpenToolCall {
                        agent_id: agent_id.clone(),
                        tool_name: tool_name.clone(),
                        step: *step,
                        started_at: Instant::now(),
                    },
                );
            }
            BusMessage::ToolResponse { request_id, .. } => {
                self.open.remove(request_id);
            }
            _ => {}
        }
    }

    /// Iterator over every open call (used by the per-agent query helpers).
    pub fn open_calls(&self) -> impl Iterator<Item = &OpenToolCall> {
        self.open.values()
    }
}
