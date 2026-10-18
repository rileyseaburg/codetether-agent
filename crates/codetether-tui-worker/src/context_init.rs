//! Construct channels' runtime context without starting background work.
use crate::{WorkerBridgeCmd, context::Context, identity::Identity};
use codetether_a2a_worker_core::{CognitionHeartbeatConfig, HeartbeatState};
use codetether_bus_core::AgentBus;
use codetether_worker_events::IncomingTask;
use reqwest::Client;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::{Mutex, mpsc};
impl Context {
    pub(crate) fn new(
        identity: Identity,
        bus: Arc<AgentBus>,
        cmd_rx: mpsc::Receiver<WorkerBridgeCmd>,
        task_tx: mpsc::Sender<IncomingTask>,
    ) -> Self {
        // Create shared state
        let client = Client::new();
        let processing = Arc::new(Mutex::new(HashSet::<String>::new()));
        let heartbeat_state =
            HeartbeatState::new(identity.worker_id.clone(), identity.worker_name.clone());
        let cognition_config = CognitionHeartbeatConfig::from_env();
        let codebases = vec![
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|_| ".".to_string()),
        ];

        Self {
            identity,
            bus,
            cmd_rx,
            task_tx,
            client,
            processing,
            heartbeat_state,
            cognition_config,
            codebases,
        }
    }
}
