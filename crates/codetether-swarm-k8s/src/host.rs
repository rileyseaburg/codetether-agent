//! Native executor settings, cache access and event delivery.
use codetether_session_control::ControlHandle;
use codetether_swarm_core::{SubTask, SubTaskResult, SwarmConfig, SwarmEvent};
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;
/// Cache operations executed under the existing executor cache mutex.
/// # Implementors
/// Implement with the application's existing task-result cache.
pub trait Cache: Send {
    /// Read and account for a task cache hit or miss.
    fn get(&mut self, task: &SubTask) -> impl Future<Output = Option<SubTaskResult>> + Send;
    /// Write a successful task result.
    /// # Errors
    /// Returns the underlying cache storage error.
    fn put(
        &mut self,
        task: &SubTask,
        result: &SubTaskResult,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
/// Read-only executor interface for one Kubernetes stage.
/// # Implementors
/// Supply the existing configuration, control handle, shared cache and event sink.
pub trait Host: Sync {
    /// Application cache implementation.
    type Cache: Cache;
    /// Current swarm configuration.
    fn config(&self) -> &SwarmConfig;
    /// Shared cancellation/pause/input state, if installed.
    fn control(&self) -> Option<&ControlHandle>;
    /// Shared task-result cache, if enabled.
    fn cache(&self) -> Option<&Arc<Mutex<Self::Cache>>>;
    /// Publish the same lifecycle event to the executor's configured sinks.
    fn try_send_event(&self, event: SwarmEvent);
}
