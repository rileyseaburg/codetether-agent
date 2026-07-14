//! Construction of Kubernetes-stage runtime state.

use super::super::SwarmExecutor;
use super::state::State;
use crate::k8s::K8sManager;
use crate::swarm::{CollapseController, CollapsePolicy, SubTask};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

impl<'a> State<'a> {
    pub fn new(
        executor: &'a SwarmExecutor,
        swarm: &'a str,
        k8s: K8sManager,
        provider: String,
        model: String,
        tasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
    ) -> Self {
        Self {
            executor,
            swarm_id: swarm,
            k8s,
            provider,
            model,
            budget: executor.config.k8s_pod_budget.max(1),
            cache_tasks: super::super::cache_result::read_only_tasks(&tasks),
            pending: tasks.into(),
            active: HashMap::new(),
            names: HashMap::new(),
            results: Vec::new(),
            kill_reasons: HashMap::new(),
            promoted: None,
            collapse: CollapseController::new(CollapsePolicy::default()),
            completed,
        }
    }
}
