//! Mutable state for one top-level swarm run.

use super::super::SwarmExecutor;
use crate::swarm::{Orchestrator, SubTask, SubTaskResult, SwarmArtifact};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

pub(super) struct Run<'a> {
    pub executor: &'a SwarmExecutor,
    pub orchestrator: Orchestrator,
    pub subtasks: Vec<SubTask>,
    pub results: Vec<SubTaskResult>,
    pub artifacts: Vec<SwarmArtifact>,
    pub completed: Arc<RwLock<HashMap<String, String>>>,
    pub failed: HashSet<String>,
    pub swarm_id: String,
    pub started_at: Instant,
}

impl<'a> Run<'a> {
    pub fn new(
        executor: &'a SwarmExecutor,
        orchestrator: Orchestrator,
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
