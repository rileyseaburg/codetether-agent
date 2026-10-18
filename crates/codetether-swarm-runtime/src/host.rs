//! Native services and backend invocation used by the swarm run lifecycle.
use codetether_session_control::ControlHandle;
use codetether_swarm_core::{
    SubTask, SubTaskResult, SwarmConfig, SwarmEvent, result_store::ResultStore,
};
use codetether_swarm_orchestrator::Orchestrator;
use codetether_telemetry::SwarmTelemetryCollector;
use std::{collections::HashMap, future::Future, sync::Arc};
use tokio::sync::RwLock;
/// Shared executor resources used by planning and result publication.
/// # Implementors
/// Return the existing native resources without replacing their ownership.
pub trait Services: Sync {
    /// Provider-registry host used to construct the orchestrator.
    type Planning: codetether_swarm_orchestrator::Host;
    /// Current swarm configuration.
    fn config(&self) -> &SwarmConfig;
    /// Optional shared runtime control handle.
    fn control(&self) -> Option<&ControlHandle>;
    /// Shared result store.
    fn result_store(&self) -> &Arc<ResultStore>;
    /// Shared telemetry collector.
    fn telemetry(&self) -> &Arc<SwarmTelemetryCollector>;
    /// Publish a run event through the existing executor sinks.
    fn try_send_event(&self, event: SwarmEvent);
}
/// Execute a dependency stage through its native backend binding.
/// # Implementors
/// Preserve the existing local and Kubernetes stage adapters.
pub trait Stages: Services {
    /// Run tasks using the local backend.
    /// # Errors
    /// Returns native stage setup or execution errors.
    fn local(
        &self,
        orchestrator: &Orchestrator<Self::Planning>,
        tasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
        swarm: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<SubTaskResult>>> + Send;
    /// Run tasks using the Kubernetes backend.
    /// # Errors
    /// Returns native stage setup or execution errors.
    fn kubernetes(
        &self,
        orchestrator: &Orchestrator<Self::Planning>,
        tasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
        swarm: &str,
    ) -> impl Future<Output = anyhow::Result<Vec<SubTaskResult>>> + Send;
}
/// Complete swarm-run interface.
/// # Implementors
/// Every provider of resources and both stage backends implements this trait.
pub trait Host: Services + Stages {}
impl<T: Services + Stages> Host for T {}
