use crate::swarm::SubTask;
use std::collections::{HashMap, HashSet};

pub(super) struct DependencyGraph<'a> {
    pub ids: HashSet<&'a str>,
    pub edges: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> DependencyGraph<'a> {
    pub fn from_subtasks(subtasks: &'a [SubTask]) -> Self {
        let ids = subtasks.iter().map(|subtask| subtask.id.as_str()).collect();
        let edges = subtasks
            .iter()
            .map(|subtask| (subtask.id.as_str(), dependencies(subtask)))
            .collect();
        Self { ids, edges }
    }
}

fn dependencies(subtask: &SubTask) -> Vec<&str> {
    subtask.dependencies.iter().map(String::as_str).collect()
}
