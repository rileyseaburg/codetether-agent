//! Application provider lookup and agent-job execution for local stages.
use crate::ExecutorHost;
use codetether_provider_core::Provider;
use codetether_swarm_core::SubTaskResult;
use codetether_swarm_local_job::Job;
use std::{future::Future, sync::Arc};
/// Native operations required by local-stage service bindings.
/// # Implementors
/// Applications select their provider registry and preserve their agent job runtime.
pub trait LocalStageHost: ExecutorHost {
    /// Provider registry retained by the stage's planner.
    type Providers: Send + Sync;
    /// Enumerate provider names in the registry's existing order.
    fn list(providers: &Self::Providers) -> Vec<&str>;
    /// Borrow a provider through the registry's existing lookup.
    fn get(providers: &Self::Providers, name: &str) -> Option<Arc<dyn Provider>>;
    /// Execute a fully prepared local job in the application agent runtime.
    fn run_job(
        job: Job,
    ) -> impl Future<Output = (String, anyhow::Result<SubTaskResult>)> + Send + 'static;
}
