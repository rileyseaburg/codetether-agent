//! Native services used by local-stage coordination.
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use codetether_session_control::ControlHandle;
use codetether_swarm_core::{
    SubTask, SubTaskResult, SwarmConfig, SwarmEvent, result_store::ResultStore,
};
use codetether_swarm_local_job::Job;
use std::{future::Future, sync::Arc};
use tokio::sync::mpsc;

/// Services operations supplied by the native local-stage executor.
/// # Implementors
/// Preserve existing native state access and operation semantics.
pub trait Services {
    /// Current executor settings.
    fn config(&self) -> &SwarmConfig;

    /// Optional runtime cancellation/input handle.
    fn control(&self) -> Option<&ControlHandle>;

    /// Event sender used for child lifecycle updates.
    fn events(&self) -> &Option<mpsc::Sender<SwarmEvent>>;

    /// Optional shared agent bus.
    fn bus(&self) -> &Option<Arc<AgentBus>>;

    /// Shared result store passed to child jobs.
    fn result_store(&self) -> &Arc<ResultStore>;

    /// Send the executor's ordinary event, including bus publication.
    fn try_send_event(&self, event: SwarmEvent);
}

/// Providers operations supplied by the native local-stage executor.
/// # Implementors
/// Preserve existing native state access and operation semantics.
pub trait Providers {
    /// Provider registry retained by the caller for the entire stage.
    type Providers: Sync;

    /// Retrieve a provider without altering registry state.
    fn get(providers: &Self::Providers, name: &str) -> Option<Arc<dyn Provider>>;

    /// Apply the existing optional delegation policy under its native lock.
    fn choose(
        &self,
        providers: &Self::Providers,
        specialty: &str,
        default_provider: &str,
        default_model: &str,
    ) -> (String, String);

    /// Feed completed work back into native delegation state.
    fn outcome(&self, provider: &str, specialty: &str, success: bool);
}

/// Cache operations supplied by the native local-stage executor.
/// # Implementors
/// Preserve existing native state access and operation semantics.
pub trait Cache {
    /// Read a task cache entry under the existing cache mutex.
    fn cache_get(&self, task: &SubTask) -> impl Future<Output = Option<SubTaskResult>> + Send;

    /// Write a cache entry, or succeed immediately when no cache is installed.
    /// # Errors
    /// Returns the native cache write error.
    fn cache_put(
        &self,
        task: &SubTask,
        result: &SubTaskResult,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Execution operations supplied by the native local-stage executor.
/// # Implementors
/// Preserve existing native state access and operation semantics.
pub trait Execution {
    /// Run one owned child job in the native agent runtime.
    fn run_job(
        job: Job,
    ) -> impl Future<Output = (String, anyhow::Result<SubTaskResult>)> + Send + 'static;
}

/// Complete native service set for local-stage execution.
/// # Implementors
/// Every type implementing the four focused service interfaces implements this trait.
pub trait Host: Services + Providers + Cache + Execution + Sync {}
impl<T: Services + Providers + Cache + Execution + Sync> Host for T {}
