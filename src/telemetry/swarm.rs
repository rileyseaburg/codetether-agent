//! Multi-agent swarm telemetry.
//!
//! Tracks the lifecycle of one swarm operation: start time, progress, and
//! final outcome. Emits structured tracing at each transition.

use chrono::{DateTime, Utc};
use tokio::sync::Mutex;

use super::metrics::TelemetryMetrics;

/// Live collector for a running swarm. Methods are async because the
/// underlying fields use [`tokio::sync::Mutex`] — swarm events are rare,
/// so lock contention is not a concern.
#[derive(Debug, Default)]
pub struct SwarmTelemetryCollector {
    task_id: Mutex<Option<String>>,
    agent_count: Mutex<usize>,
    completed: Mutex<usize>,
    total: Mutex<usize>,
    start_time: Mutex<Option<DateTime<Utc>>>,
}

impl SwarmTelemetryCollector {
    /// Construct an empty collector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record swarm start. `_strategy` is logged downstream.
    pub async fn start_swarm(&self, task_id: &str, agent_count: usize, _strategy: &str) {
        *self.task_id.lock().await = Some(task_id.to_string());
        *self.agent_count.lock().await = agent_count;
        *self.start_time.lock().await = Some(Utc::now());
        tracing::info!(task_id, agent_count, "Swarm started");
    }

    /// Update `(completed, total)` progress counters.
    pub async fn record_progress(&self, completed: usize, total: usize) {
        *self.completed.lock().await = completed;
        *self.total.lock().await = total;
    }

    /// Log a named latency sample (e.g. per-stage timing).
    pub async fn record_swarm_latency(&self, label: &str, duration: std::time::Duration) {
        tracing::debug!(
            label,
            duration_ms = duration.as_millis(),
            "Swarm latency recorded"
        );
    }

    /// Finalize the swarm and produce a [`TelemetryMetrics`] summary.
    pub async fn complete_swarm(&self, success: bool) -> TelemetryMetrics {
        let duration = self
            .start_time
            .lock()
            .await
            .map(|s| (Utc::now() - s).num_milliseconds() as u64)
            .unwrap_or(0);

        let completed = *self.completed.lock().await;
        let total = *self.total.lock().await;

        tracing::info!(
            success,
            completed,
            total,
            duration_ms = duration,
            "Swarm completed"
        );

        TelemetryMetrics {
            tool_invocations: total as u64,
            successful_operations: if success { completed as u64 } else { 0 },
            failed_operations: if !success {
                (total.saturating_sub(completed)) as u64
            } else {
                0
            },
            total_tokens: 0,
            avg_latency_ms: duration as f64,
        }
    }
}
