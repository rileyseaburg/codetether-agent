//! Heartbeat state shared across worker runtimes.

use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerStatus {
    Idle,
    Processing,
}

impl WorkerStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Processing => "processing",
        }
    }
}

#[derive(Clone)]
pub struct HeartbeatState {
    pub worker_id: String,
    pub agent_name: String,
    pub status: Arc<Mutex<WorkerStatus>>,
    pub active_task_count: Arc<Mutex<usize>>,
    pub sub_agents: Arc<Mutex<HashSet<String>>>,
}

impl HeartbeatState {
    pub fn new(worker_id: String, agent_name: String) -> Self {
        Self {
            worker_id,
            agent_name,
            status: Arc::new(Mutex::new(WorkerStatus::Idle)),
            active_task_count: Arc::new(Mutex::new(0)),
            sub_agents: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}
