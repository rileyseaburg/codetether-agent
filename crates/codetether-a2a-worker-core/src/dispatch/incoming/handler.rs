//! Native execution hook for tasks admitted by the worker dispatch policy.
use crate::initialization::WorkerTaskRuntime;
use serde_json::Value;
use std::future::Future;

/// Execute one admitted worker task with its existing runtime services.
///
/// # Implementors
/// Applications provide their HTTP client, event bus, and existing task handler.
pub trait Handler: 'static {
    /// Shared HTTP client; cloning retains its connection-pool semantics.
    type Client: Clone + Send + Sync + 'static;
    /// Shared worker event bus.
    type Bus: Send + Sync + 'static;
    /// Execute the task, propagating failures for dispatch logging.
    fn handle(
        runtime: &WorkerTaskRuntime<Self::Client, Self::Bus>,
        task: &Value,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
