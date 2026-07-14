//! Mutable resources for one Kubernetes-backed stage.

use super::super::SwarmExecutor;
use crate::k8s::K8sManager;
use crate::swarm::{CollapseController, SubTask, SubTaskResult};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

#[derive(Clone)]
pub(super) struct ActiveBranch {
    pub branch: String,
    pub started_at: Instant,
}

pub(super) struct State<'a> {
    pub executor: &'a SwarmExecutor,
    pub swarm_id: &'a str,
    pub k8s: K8sManager,
    pub provider: String,
    pub model: String,
    pub budget: usize,
    pub pending: VecDeque<SubTask>,
    pub active: HashMap<String, ActiveBranch>,
    pub names: HashMap<String, String>,
    pub results: Vec<SubTaskResult>,
    pub kill_reasons: HashMap<String, String>,
    pub promoted: Option<String>,
    pub collapse: CollapseController,
    pub cache_tasks: HashMap<String, SubTask>,
    pub completed: Arc<RwLock<HashMap<String, String>>>,
}
