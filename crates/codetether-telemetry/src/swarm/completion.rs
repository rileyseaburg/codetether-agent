//! Focused operations for the parent telemetry type.

use super::*;

impl SwarmTelemetryCollector {
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
