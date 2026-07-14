//! Mutable resources used while executing one local stage.

use super::super::SwarmExecutor;
use crate::provider::Provider;
use crate::swarm::{SubTask, SubTaskResult};
use crate::worktree::{WorktreeInfo, WorktreeManager};
use futures::stream::FuturesUnordered;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::{AbortHandle, JoinHandle};

pub(super) type AgentJoin = JoinHandle<(String, anyhow::Result<SubTaskResult>)>;

pub(super) struct State<'a> {
    pub executor: &'a SwarmExecutor,
    pub swarm_id: &'a str,
    pub prior_results: Arc<RwLock<HashMap<String, String>>>,
    pub providers: &'a crate::provider::ProviderRegistry,
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
