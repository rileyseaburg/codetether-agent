//! Per-instance rolling telemetry metrics.
//!
//! Prefer the global [`crate::telemetry::TOKEN_USAGE`] / `TOOL_EXECUTIONS`
//! singletons for process-wide counts. Use [`Telemetry`] when you need a
//! scoped collector whose lifetime is tied to a specific agent instance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;

/// Snapshot-able metrics for an agent instance.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::TelemetryMetrics;
///
/// let m = TelemetryMetrics::default();
/// assert_eq!(m.tool_invocations, 0);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    /// Total tool invocations.
    pub tool_invocations: u64,
    /// Tool invocations that returned `Ok`.
    pub successful_operations: u64,
    /// Tool invocations that returned an error.
    pub failed_operations: u64,
    /// Sum of tokens consumed across all invocations.
    pub total_tokens: u64,
    /// Running mean latency in ms.
    pub avg_latency_ms: f64,
}

/// Per-instance telemetry tracker with an async-friendly rolling mean.
#[derive(Debug)]
pub struct Telemetry {
    metrics: Mutex<TelemetryMetrics>,
    /// Free-form instance metadata (agent id, tenant, etc).
    pub metadata: HashMap<String, String>,
}

impl Telemetry {
    /// Build an empty tracker.
    pub fn new() -> Self {
        Self {
            metrics: Mutex::new(TelemetryMetrics::default()),
            metadata: HashMap::new(),
        }
    }

    /// Record one invocation, updating the rolling mean latency.
    pub async fn record_tool_invocation(&self, success: bool, latency_ms: u64, tokens: u64) {
        let mut metrics = self.metrics.lock().await;
        metrics.tool_invocations += 1;
        if success {
            metrics.successful_operations += 1;
        } else {
            metrics.failed_operations += 1;
        }
        metrics.total_tokens += tokens;
        let n = metrics.tool_invocations as f64;
        metrics.avg_latency_ms = metrics.avg_latency_ms * (n - 1.0) / n + latency_ms as f64 / n;
    }

    /// Clone the current metrics.
    pub async fn get_metrics(&self) -> TelemetryMetrics {
        self.metrics.lock().await.clone()
    }

    /// Placeholder for future per-instance swarm telemetry.
    pub async fn start_swarm(&self, _task_id: &str, _agent_count: usize) {}

    /// Placeholder for future per-instance swarm progress tracking.
    pub async fn record_swarm_progress(&self, _task_id: &str, _completed: usize, _total: usize) {}

    /// Placeholder: returns the current metrics regardless of `_success`.
    pub async fn complete_swarm(&self, _success: bool) -> TelemetryMetrics {
        self.metrics.lock().await.clone()
    }
}

impl Default for Telemetry {
    fn default() -> Self {
        Self::new()
    }
}
