//! Core state owned by the swarm executor.

use crate::bus::AgentBus;
use crate::session::delegation::DelegationState;
use crate::swarm::{ResultStore, SwarmCache, SwarmConfig, SwarmControl};
use crate::{agent::Agent, telemetry::SwarmTelemetryCollector};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

/// Coordinates providers, caches, worktrees, and runtime events for a swarm.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::{SwarmConfig, SwarmExecutor};
/// let executor = SwarmExecutor::new(SwarmConfig::default());
/// assert!(executor.control().is_none());
/// ```
pub struct SwarmExecutor {
    pub(super) config: SwarmConfig,
    pub(super) coordinator_agent: Option<Arc<tokio::sync::Mutex<Agent>>>,
    pub(super) event_tx: Option<mpsc::Sender<crate::tui::swarm_view::SwarmEvent>>,
    pub(super) telemetry: Arc<SwarmTelemetryCollector>,
    pub(super) cache: Option<Arc<tokio::sync::Mutex<SwarmCache>>>,
    pub(super) result_store: Arc<ResultStore>,
    pub(super) bus: Option<Arc<AgentBus>>,
    pub(super) delegation: Option<Arc<Mutex<DelegationState>>>,
    pub(super) control: Option<SwarmControl>,
}
