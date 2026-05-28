//! Shared initialized state reused by worker entrypoints.

use std::{collections::HashSet, sync::Arc};

use tokio::sync::Mutex;

use crate::{bus::AgentBus, cli::A2aArgs};

use super::{CognitionHeartbeatConfig, HeartbeatState, WorkerTaskRuntime, task_timeline};

pub(super) struct WorkerContext {
    pub(super) args: A2aArgs,
    pub(super) server: String,
    pub(super) name: String,
    pub(super) worker_id: String,
    pub(super) shared_codebases: Arc<Mutex<Vec<String>>>,
    pub(super) processing: Arc<Mutex<HashSet<String>>>,
    pub(super) cognition_heartbeat: CognitionHeartbeatConfig,
    pub(super) heartbeat_state: HeartbeatState,
    pub(super) bus: Arc<AgentBus>,
    pub(super) task_progress: Arc<Mutex<task_timeline::TaskProgressState>>,
    pub(super) task_runtime: WorkerTaskRuntime,
}
