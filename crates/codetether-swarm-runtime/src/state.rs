//! Mutable state for one top-level swarm run.

use crate::Host;
use codetether_swarm_core::{SubTask, SubTaskResult, SwarmArtifact};
use codetether_swarm_orchestrator::Orchestrator;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

pub(super) struct Run<'a, H: Host> {
    pub executor: &'a H,
    pub orchestrator: Orchestrator<H::Planning>,
    pub subtasks: Vec<SubTask>,
    pub results: Vec<SubTaskResult>,
    pub artifacts: Vec<SwarmArtifact>,
    pub completed: Arc<RwLock<HashMap<String, String>>>,
    pub failed: HashSet<String>,
    pub swarm_id: String,
    pub started_at: Instant,
}

impl<'a, H: Host> Run<'a, H> {
    pub fn new(
        executor: &'a H,
        orchestrator: Orchestrator<H::Planning>,
        subtasks: Vec<SubTask>,
    ) -> Self {
        Self {
            executor,
            orchestrator,
            subtasks,
            results: Vec::new(),
            artifacts: Vec::new(),
            completed: Arc::new(RwLock::new(HashMap::new())),
            failed: HashSet::new(),
            swarm_id: uuid::Uuid::new_v4().to_string(),
            started_at: Instant::now(),
        }
    }
}
