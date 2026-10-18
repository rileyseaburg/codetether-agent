//! Runtime dependencies and configuration owned by a swarm executor.
use codetether_bus_core::AgentBus;
use codetether_context_policy::delegation::DelegationState;
use codetether_session_control::ControlHandle;
use codetether_swarm_core::{SwarmConfig, SwarmEvent, result_store::ResultStore};
use codetether_telemetry::SwarmTelemetryCollector;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// Executor state with application-supplied coordinator and cache types.
/// # Examples
/// ```
/// let state = codetether_swarm_runtime::ExecutorState::<(), ()>::new(Default::default());
/// assert!(state.control.is_none());
/// assert!(state.cache.is_none());
/// ```
pub struct ExecutorState<A, C> {
    /// Execution limits, models, and retry settings.
    pub config: SwarmConfig,
    /// Optional coordinator agent.
    pub coordinator_agent: Option<Arc<tokio::sync::Mutex<A>>>,
    /// Optional live event destination.
    pub event_tx: Option<mpsc::Sender<SwarmEvent>>,
    /// Shared telemetry collector.
    pub telemetry: Arc<SwarmTelemetryCollector>,
    /// Optional synchronized result cache.
    pub cache: Option<Arc<tokio::sync::Mutex<C>>>,
    /// Shared subtask result store.
    pub result_store: Arc<ResultStore>,
    /// Optional inter-agent message bus.
    pub bus: Option<Arc<AgentBus>>,
    /// Optional adaptive provider delegation state.
    pub delegation: Option<Arc<Mutex<DelegationState>>>,
    /// Optional pause and cancellation control.
    pub control: Option<ControlHandle>,
}

impl<A, C> ExecutorState<A, C> {
    /// Create executor state with fresh telemetry and a shared result store.
    /// # Arguments
    /// * `config` — Execution limits and provider settings.
    /// # Returns
    /// State with optional services unset, ready for application attachment.
    /// # Examples
    /// See [ExecutorState] for construction with generic service types.
    pub fn new(config: SwarmConfig) -> Self {
        Self {
            config,
            coordinator_agent: None,
            event_tx: None,
            telemetry: Arc::new(SwarmTelemetryCollector::default()),
            cache: None,
            result_store: ResultStore::new_arc(),
            bus: None,
            delegation: None,
            control: None,
        }
    }
}
