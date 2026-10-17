//! replay operations for the shared bus runtime.
use super::*;
impl AgentBus {
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
