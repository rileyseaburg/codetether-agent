//! Durable, replayable log abstraction for coordination messages.
//!
//! This is the **primary** transport for at-least-once coordination traffic.
//! The in-process [`crate::bus::AgentBus`] broadcast channel is demoted to a
//! lossy live-presence cache; anything that must survive a restart or be
//! replayed by a late-joining agent is appended here.
//!
//! The default implementation ([`crate::bus::durable_log_file`]) is an
//! append-only JSONL store. The same trait can be backed by NATS JetStream,
//! Redis Streams, or Kafka for multi-process scale-out without touching
//! callers.

use super::BusEnvelope;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Partition under which non-partitioned messages are stored/replayed.
pub const GLOBAL_PARTITION: &str = "_global";

/// An append-only, partitioned, replayable log of [`BusEnvelope`]s.
#[async_trait]
pub trait DurableLog: Send + Sync {
    /// Append `env` and return its monotonic offset within its partition.
    ///
    /// Callers may retry on error; the log must tolerate at-least-once
    /// semantics (duplicate envelope ids are possible after a retry).
    async fn append(&self, env: &BusEnvelope) -> Result<u64>;

    /// Replay all envelopes in `partition` with offset strictly greater than
    /// `after`. Pass `0` to replay from the beginning.
    async fn tail(&self, partition: &str, after: u64) -> Result<Vec<BusEnvelope>>;
}

/// Resolve the storage partition for an envelope.
///
/// Uses [`BusMessage::partition_key`](crate::bus::BusMessage::partition_key)
/// when present, else [`GLOBAL_PARTITION`].
pub fn partition_of(env: &BusEnvelope) -> &str {
    env.message.partition_key().unwrap_or(GLOBAL_PARTITION)
}

/// Fire-and-forget append of a durable envelope on a background task.
///
/// No-op when `log` is `None` or the message is presence-only. Keeps the
/// synchronous `publish` path non-blocking; append failures are logged.
pub fn spawn_append_if_durable(log: Option<&Arc<dyn DurableLog>>, envelope: &BusEnvelope) {
    let Some(log) = log else { return };
    if !envelope.message.is_durable() {
        return;
    }
    let log = Arc::clone(log);
    let env = envelope.clone();
    tokio::spawn(async move {
        if let Err(e) = log.append(&env).await {
            tracing::error!(error = %e, topic = %env.topic, "durable append failed");
        }
    });
}
