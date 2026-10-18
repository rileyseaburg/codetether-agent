//! Owned inputs for one locally spawned sub-agent.

use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use codetether_swarm_core::SwarmEvent;
use codetether_swarm_core::result_store::ResultStore;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};

/// Owned provider, workspace and limits for a single local swarm task.
/// # Examples
/// Obtain jobs from a stage builder; [crate::execute] consumes them.
pub struct Job {
    /// Subtask identity.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Task instructions.
    pub instruction: String,
    /// Agent specialty.
    pub specialty: String,
    /// Prior-work context.
    pub context: String,
    /// Selected provider.
    pub provider: Arc<dyn Provider>,
    /// Selected provider model.
    pub model: String,
    /// Whether the job uses read-only tools.
    pub read_only: bool,
    /// Whether the job verifies prior work.
    pub verification: bool,
    /// Whether file changes are required.
    pub expects_changes: bool,
    /// Enforced workspace root.
    pub workspace: PathBuf,
    /// Shared task-result store.
    pub result_store: Arc<ResultStore>,
    /// Optional event bus.
    pub bus: Option<Arc<AgentBus>>,
    /// Optional lifecycle event channel.
    pub events: Option<mpsc::Sender<SwarmEvent>>,
    /// Stage concurrency limit.
    pub semaphore: Arc<Semaphore>,
    /// Delay before requesting a permit.
    pub stagger_ms: u64,
    /// Maximum agent steps.
    pub max_steps: usize,
    /// Maximum attempt duration.
    pub timeout_secs: u64,
    /// Retry count limit.
    pub max_retries: u32,
    /// Initial retry backoff.
    pub base_delay_ms: u64,
    /// Maximum retry backoff.
    pub max_delay_ms: u64,
    /// Parent runtime capability.
    pub control: Option<codetether_session_control::ControlHandle>,
}
