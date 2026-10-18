//! Claimed-task execution using native events and application sessions.
//!
//! [`Executor`] implements the lifecycle execution hook. It combines a
//! [`SessionFactory`] with the native event transport and a [`Forage`] hook,
//! then delegates execution order and outcome handling to the shared driver.
use crate::{WorkerTaskRuntime, special::Forage};
use codetether_a2a_worker_core::{
    outcome::TaskInner, task_driver::claimed::Sessions, task_timeline::TaskTimeline,
};
use codetether_provenance::ClaimProvenance;
use std::marker::PhantomData;
mod events;
mod sessions;
mod special;

/// Construct the session adapter borrowed by a claimed-task execution.
///
/// # Implementors
/// Applications supply their concrete session preparation and execution adapter.
pub trait SessionFactory: 'static {
    /// Session operations that borrow the worker runtime for one execution.
    type Sessions<'a>: Sessions + Send;
    /// Create the adapter without performing session preparation yet.
    ///
    /// # Arguments
    /// * `runtime` - Worker identity and services borrowed for this execution.
    ///
    /// # Returns
    /// An adapter implementing the shared claimed-task session contract.
    fn sessions(runtime: &WorkerTaskRuntime) -> Self::Sessions<'_>;
}

/// Compose application sessions and forage with the native worker lifecycle.
///
/// # Examples
/// Select this type as the executor of a native task dispatcher.
/// ```no_run
/// # async fn example<S: codetether_native_worker::claimed::SessionFactory, F: codetether_native_worker::special::Forage + 'static>(runtime: &codetether_native_worker::WorkerTaskRuntime) -> anyhow::Result<()> {
/// use codetether_native_worker::{claimed::Executor, lifecycle::TaskDispatcher, pending::fetch_pending_tasks};
/// fetch_pending_tasks::<TaskDispatcher<Executor<S, F>>>(runtime).await
/// # }
/// ```
pub struct Executor<S, F>(PhantomData<fn() -> (S, F)>);

struct Native<'a, S, F> {
    runtime: &'a WorkerTaskRuntime,
    sessions: S,
    forage: PhantomData<fn() -> F>,
}

impl<S: SessionFactory, F: Forage + 'static> crate::lifecycle::Executor for Executor<S, F> {
    async fn execute(
        runtime: &WorkerTaskRuntime,
        task: &serde_json::Value,
        task_id: &str,
        title: &str,
        provenance: &ClaimProvenance,
        provider_keys: Option<serde_json::Value>,
        timeline: &mut TaskTimeline,
    ) -> TaskInner {
        codetether_a2a_worker_core::task_driver::claimed::execute(
            &Native::<_, F> {
                runtime,
                sessions: S::sessions(runtime),
                forage: PhantomData,
            },
            task,
            task_id,
            title,
            provenance,
            provider_keys,
            timeline,
        )
        .await
    }
}
