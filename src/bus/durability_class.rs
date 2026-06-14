//! Delivery-semantics classifier for [`BusMessage`].
//!
//! Splits bus traffic into two classes so the bus can dual-write:
//! - **Durable** coordination/state messages need at-least-once delivery and
//!   replay (task state, handoffs, work assignment, shared results).
//! - **Presence** messages (heartbeats, live thinking, voice fragments) are
//!   fine to ride the lossy in-process broadcast channel.
//!
//! See [`crate::bus::durable_log`] for the replay substrate.

use super::BusMessage;

impl BusMessage {
    /// Whether this message must be persisted to the durable log.
    ///
    /// Coordination and state-transition messages return `true`; ephemeral
    /// presence/telemetry returns `false`.
    pub fn is_durable(&self) -> bool {
        matches!(
            self,
            BusMessage::TaskUpdate { .. }
                | BusMessage::ArtifactUpdate { .. }
                | BusMessage::AgentMessage { .. }
                | BusMessage::SharedResult { .. }
                | BusMessage::ToolRequest { .. }
                | BusMessage::ToolResponse { .. }
                | BusMessage::RalphHandoff { .. }
                | BusMessage::RalphProgress { .. }
                | BusMessage::RalphLearning { .. }
        )
    }

    /// Partition key for per-workstream replay, or `None` when the message is
    /// not partitioned (replayed under the global partition).
    pub fn partition_key(&self) -> Option<&str> {
        match self {
            BusMessage::TaskUpdate { task_id, .. }
            | BusMessage::ArtifactUpdate { task_id, .. } => Some(task_id),
            BusMessage::RalphHandoff { prd_id, .. }
            | BusMessage::RalphProgress { prd_id, .. }
            | BusMessage::RalphLearning { prd_id, .. } => Some(prd_id),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "durability_class_tests.rs"]
mod tests;
