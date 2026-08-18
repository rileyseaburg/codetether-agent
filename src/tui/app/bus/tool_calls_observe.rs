//! Bounded mutation of the open tool-call tracker.

use std::time::Instant;

use crate::bus::BusMessage;

use super::{OpenToolCall, ToolCallTracker};

pub(crate) const MAX_OPEN_CALLS: usize = 256;

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
            } => self.insert(
                request_id,
                OpenToolCall {
                    agent_id: agent_id.clone(),
                    tool_name: tool_name.clone(),
                    step: *step,
                    started_at: Instant::now(),
                },
            ),
            BusMessage::ToolResponse { request_id, .. } => {
                self.open.remove(request_id);
            }
            _ => {}
        }
    }

    fn insert(&mut self, request_id: &str, call: OpenToolCall) {
        if self.open.len() >= MAX_OPEN_CALLS
            && !self.open.contains_key(request_id)
            && let Some(oldest) = self
                .open
                .iter()
                .min_by_key(|(_, call)| call.started_at)
                .map(|(id, _)| id.clone())
        {
            self.open.remove(&oldest);
        }
        self.open.insert(request_id.to_string(), call);
    }
}
