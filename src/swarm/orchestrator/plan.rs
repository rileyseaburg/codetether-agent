//! Validation and fallback construction for swarm plans.

use super::super::SubTask;
use super::stages;
use anyhow::Result;
use anyhow::bail;
use std::collections::{HashMap, HashSet};

pub(super) fn validate(tasks: Vec<SubTask>) -> Result<HashMap<String, SubTask>> {
    if tasks.is_empty() {
        bail!("Swarm plan has no subtasks");
    }
    let mut names = HashSet::new();
    if tasks.iter().any(|task| !names.insert(task.name.clone())) {
        bail!("Swarm plan has duplicate subtask names");
    }
    let mut plan = tasks
        .into_iter()
        .map(|task| (task.id.clone(), task))
        .collect::<HashMap<_, _>>();
    stages::assign(&mut plan)?;
    Ok(plan)
}

pub(super) fn single(instruction: &str) -> HashMap<String, SubTask> {
    let task = SubTask::new("Main Task", instruction);
    HashMap::from([(task.id.clone(), task)])
}

#[cfg(test)]
#[path = "plan_tests.rs"]
mod tests;
