//! Durable-log methods for [`AgentBus`] kept out of the oversized `mod.rs`.
//!
//! Provides replay so a late-joining or restarted agent can deterministically
//! catch up on a task/PRD partition from the durable coordination log.

use super::{AgentBus, BusEnvelope, durable_log};
use std::sync::Arc;

impl AgentBus {
    /// Attach a durable log so coordination messages are persisted for replay.
    ///
    /// Builder-style; presence messages ([`super::BusMessage::is_durable`] is
    /// `false`) are never written to the log.
    pub fn with_durable_log(mut self, log: Arc<dyn durable_log::DurableLog>) -> Self {
        self.durable = Some(log);
        self
    }

    /// Replay durable coordination messages for `partition` (a task/PRD id)
    /// whose offset is strictly greater than `after`.
    ///
    /// Returns an empty vector when no durable log is attached. Pass `0` for
    /// `after` to replay the partition from the beginning.
    pub async fn replay(&self, partition: &str, after: u64) -> Vec<BusEnvelope> {
        match &self.durable {
            Some(log) => log.tail(partition, after).await.unwrap_or_default(),
            None => Vec::new(),
        }
    }
}
