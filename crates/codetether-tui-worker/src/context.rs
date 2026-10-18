//! Runtime state shared by registration, heartbeat and command handling.
use crate::{WorkerBridgeCmd, identity::Identity};
use codetether_a2a_worker_core::{CognitionHeartbeatConfig, HeartbeatState};
use codetether_bus_core::AgentBus;
use codetether_worker_events::IncomingTask;
use reqwest::Client;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::{Mutex, mpsc};
pub(crate) struct Context {
    pub identity: Identity,
    pub client: Client,
    pub processing: Arc<Mutex<HashSet<String>>>,
    pub heartbeat_state: HeartbeatState,
    pub cognition_config: CognitionHeartbeatConfig,
    pub codebases: Vec<String>,
    pub bus: Arc<AgentBus>,
    pub cmd_rx: mpsc::Receiver<WorkerBridgeCmd>,
    pub task_tx: mpsc::Sender<IncomingTask>,
}
