//! Owned inputs for one locally spawned sub-agent.

use crate::bus::AgentBus;
use crate::provider::Provider;
use crate::swarm::ResultStore;
use crate::tui::swarm_view::SwarmEvent;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};

pub(super) struct Job {
    pub id: String,
    pub name: String,
    pub instruction: String,
    pub specialty: String,
    pub context: String,
    pub provider: Arc<dyn Provider>,
    pub model: String,
    pub read_only: bool,
    pub verification: bool,
    pub expects_changes: bool,
    pub workspace: PathBuf,
    pub result_store: Arc<ResultStore>,
    pub bus: Option<Arc<AgentBus>>,
    pub events: Option<mpsc::Sender<SwarmEvent>>,
    pub semaphore: Arc<Semaphore>,
    pub stagger_ms: u64,
    pub max_steps: usize,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}
