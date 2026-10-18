//! Construction of Kubernetes-stage runtime state.

use super::state::State;
use crate::Host;
use codetether_k8s::K8sManager;
use codetether_swarm_core::SubTask;
use codetether_swarm_core::collapse::{CollapseController, CollapsePolicy};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

impl<'a, H: Host> State<'a, H> {
    pub fn new(
        executor: &'a H,
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
            budget: executor.config().k8s_pod_budget.max(1),
            cache_tasks: codetether_swarm_core::cache::read_only_tasks(&tasks),
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
