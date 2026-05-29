//! Shared runtime state for active worker tasks.

use std::{collections::HashSet, sync::Arc};

use reqwest::Client;
use tokio::sync::Mutex;

use crate::bus::AgentBus;

use super::{AutoApprove, task_timeline};

/// Result of attempting to reserve a task-processing slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum TaskReservation {
    Reserved,
    AlreadyProcessing,
    AtCapacity,
}

/// Shared dependencies and mutable state reused across task handlers.
#[derive(Clone)]
pub(super) struct WorkerTaskRuntime {
    pub(super) client: Client,
    pub(super) server: String,
    pub(super) token: Option<String>,
    pub(super) worker_id: String,
    pub(super) agent_name: String,
    pub(super) processing: Arc<Mutex<HashSet<String>>>,
    pub(super) max_concurrent_tasks: usize,
    pub(super) auto_approve: AutoApprove,
    pub(super) bus: Arc<AgentBus>,
    pub(super) task_progress: Arc<Mutex<task_timeline::TaskProgressState>>,
    pub(super) workspace_ids: Vec<String>,
}
