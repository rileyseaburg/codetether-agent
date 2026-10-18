//! Native task admission, claiming, progress synchronization and release.
//!
//! [`TaskDispatcher`] connects pending-task and stream intake to the shared
//! capacity and targeting rules. [`handle_task`] runs the shared claim/release
//! driver using the native HTTP runtime. Applications implement [`Executor`]
//! for the session-dependent execution of an already-claimed task.
use crate::{WorkerTaskRuntime, workspace_scope::sync_timeline_to_runtime};
use anyhow::Result;
use codetether_a2a_worker_core::{
    claim::ClaimedTaskData,
    outcome::{TaskInner, TaskOutcome},
    task_driver::TaskHandlerHost,
    task_timeline::TaskTimeline,
};
use codetether_provenance::ClaimProvenance;
use std::{future::Future, marker::PhantomData};
mod claim;
mod dispatch;
mod host;
mod release;
use claim::claim_task;

/// Application execution of an already-claimed task.
///
/// # Implementors
/// Implementations delegate to session-dependent execution. Admission, claim,
/// timeout handling and release are supplied by this module's shared driver.
pub trait Executor: 'static {
    /// Execute a task with the claim's attribution, provider keys and timeline.
    ///
    /// # Arguments
    /// * `runtime` - Native worker services and identity.
    /// * `task` - Original task payload.
    /// * `task_id` - Claimed task identifier.
    /// * `title` - Human-readable task title.
    /// * `provenance` - Attribution accepted during claiming.
    /// * `provider_keys` - Optional provider credentials supplied with the claim.
    /// * `timeline` - Mutable progress timeline for the claimed task.
    ///
    /// # Returns
    /// The task result used by the shared lifecycle driver.
    ///
    /// # Errors
    /// Returns the execution error for the driver to normalize and release.
    fn execute(
        runtime: &WorkerTaskRuntime,
        task: &serde_json::Value,
        task_id: &str,
        title: &str,
        provenance: &ClaimProvenance,
        provider_keys: Option<serde_json::Value>,
        timeline: &mut TaskTimeline,
    ) -> impl Future<Output = TaskInner> + Send;
}

/// Connect worker intake to native lifecycle handling using executor `E`.
///
/// # Examples
/// The dispatcher is selected as a type; no instance is required.
/// ```no_run
/// # async fn example<E: codetether_native_worker::lifecycle::Executor>(runtime: &codetether_native_worker::WorkerTaskRuntime) -> anyhow::Result<()> {
/// use codetether_native_worker::{lifecycle::TaskDispatcher, pending::fetch_pending_tasks};
/// fetch_pending_tasks::<TaskDispatcher<E>>(runtime).await
/// # }
/// ```
pub struct TaskDispatcher<E>(PhantomData<fn() -> E>);

struct ApplicationHost<'a, E> {
    runtime: &'a WorkerTaskRuntime,
    executor: PhantomData<fn() -> E>,
}

/// Claim, execute and release a task using the native worker runtime.
///
/// # Arguments
/// * `runtime` - Shared worker services and progress state.
/// * `task` - Task payload supplied to the existing lifecycle driver.
///
/// # Returns
/// Completion of lifecycle handling, including its release operation.
///
/// # Errors
/// Propagates errors from the shared claim, execution and release driver.
///
/// # Examples
/// ```no_run
/// # async fn example<E: codetether_native_worker::lifecycle::Executor>(runtime: &codetether_native_worker::WorkerTaskRuntime, task: &serde_json::Value) -> anyhow::Result<()> {
/// codetether_native_worker::lifecycle::handle_task::<E>(runtime, task).await
/// # }
/// ```
pub async fn handle_task<E: Executor>(
    runtime: &WorkerTaskRuntime,
    task: &serde_json::Value,
) -> Result<()> {
    codetether_a2a_worker_core::task_driver::handle_task(
        &ApplicationHost::<E> {
            runtime,
            executor: PhantomData,
        },
        task,
    )
    .await
}
