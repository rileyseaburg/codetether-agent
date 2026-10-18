//! Mutable resources used while executing one local stage.

use crate::Host;
use codetether_provider_core::Provider;
use codetether_swarm_core::{SubTask, SubTaskResult};
use codetether_worktree::{WorktreeInfo, WorktreeManager};
use futures::stream::FuturesUnordered;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::{AbortHandle, JoinHandle};

pub(super) type AgentJoin = JoinHandle<(String, anyhow::Result<SubTaskResult>)>;

pub(super) struct State<'a, H: Host> {
    pub executor: &'a H,
    pub swarm_id: &'a str,
    pub prior_results: Arc<RwLock<HashMap<String, String>>>,
    pub providers: &'a H::Providers,
    pub fallback_provider: Arc<dyn Provider>,
    pub provider_name: String,
    pub model: String,
    pub workspace: std::path::PathBuf,
    pub manager: Option<Arc<WorktreeManager>>,
    pub cache_tasks: HashMap<String, SubTask>,
    pub change_expectations: HashMap<String, bool>,
    pub handles: FuturesUnordered<AgentJoin>,
    pub aborts: HashMap<String, AbortHandle>,
    pub task_ids: HashMap<tokio::task::Id, String>,
    pub active_worktrees: HashMap<String, WorktreeInfo>,
    pub all_worktrees: HashMap<String, WorktreeInfo>,
    pub ready_worktrees: HashMap<String, WorktreeInfo>,
    pub immediate_results: Vec<SubTaskResult>,
    pub completed: Vec<(SubTaskResult, Option<WorktreeInfo>)>,
    pub assignments: HashMap<String, (String, String)>,
    pub kill_reasons: HashMap<String, String>,
    pub promoted: Option<String>,
    pub semaphore: Arc<tokio::sync::Semaphore>,
}

impl<H: Host> Drop for State<'_, H> {
    fn drop(&mut self) {
        for abort in self.aborts.values() {
            abort.abort();
        }
    }
}
