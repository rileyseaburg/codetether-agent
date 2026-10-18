//! Policy-gated special-task dispatch for native workers.
//!
//! [`dispatch_special_task`] uses shared special-task classification and outcome
//! mapping with native runtime policy and repository cloning. [`Forage`] supplies
//! the application's forage executor, which still depends on its session runtime.
use crate::WorkerTaskRuntime;
use anyhow::Result;
use codetether_a2a_worker_core::{outcome::TaskInner, task_config::TaskContext};
use serde_json::{Map, Value};
use std::{future::Future, marker::PhantomData};
mod host;

/// Application forage execution required by special-task dispatch.
///
/// # Implementors
/// Applications delegate to their configured forage runtime and session host.
pub trait Forage {
    /// Execute a forage task using the accepted title, prompt, metadata and model.
    ///
    /// # Arguments
    /// * `title` - Human-readable task title.
    /// * `prompt` - Resolved task prompt.
    /// * `metadata` - Original task metadata.
    /// * `selected_model` - Optional selected model.
    ///
    /// # Returns
    /// The forage execution result text.
    ///
    /// # Errors
    /// Propagates the application's forage execution error.
    fn execute(
        title: &str,
        prompt: &str,
        metadata: &Map<String, Value>,
        selected_model: Option<String>,
    ) -> impl Future<Output = Result<String>> + Send;
}

/// Dispatch a clone or forage task through native runtime policy.
///
/// # Arguments
/// * `runtime` - Native worker identity and HTTP services.
/// * `task` - Original task payload.
/// * `title` - Resolved task title.
/// * `context` - Shared task configuration.
///
/// # Returns
/// `None` for ordinary tasks, or the shared outcome for a handled special task.
/// Execution and policy failures are encoded in that outcome.
///
/// # Examples
/// ```no_run
/// # async fn example<F: codetether_native_worker::special::Forage>(runtime: &codetether_native_worker::WorkerTaskRuntime, task: &serde_json::Value, context: &codetether_a2a_worker_core::task_config::TaskContext) {
/// let _outcome = codetether_native_worker::special::dispatch_special_task::<F>(runtime, task, "task", context).await;
/// # }
/// ```
pub async fn dispatch_special_task<F: Forage>(
    runtime: &WorkerTaskRuntime,
    task: &Value,
    title: &str,
    context: &TaskContext,
) -> Option<TaskInner> {
    codetether_a2a_worker_core::task_driver::special::dispatch_special_task(
        &host::Native::<F> {
            runtime,
            forage: PhantomData,
        },
        task,
        title,
        context,
    )
    .await
}
