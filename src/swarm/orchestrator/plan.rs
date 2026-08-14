//! Validation and fallback construction for swarm plans.

use super::super::SubTask;
use super::stages;
use anyhow::Result;
use anyhow::bail;
use std::collections::{HashMap, HashSet};

pub(super) fn validate(
    tasks: Vec<SubTask>,
    parallel_required: bool,
) -> Result<HashMap<String, SubTask>> {
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
    if parallel_required && plan.values().filter(|task| task.stage == 0).count() < 2 {
        bail!("Parallel swarm plans require at least two dependency-free root subtasks");
    }
    Ok(plan)
}

pub(super) fn single(instruction: &str) -> HashMap<String, SubTask> {
    let task = SubTask::new("Main Task", instruction);
    HashMap::from([(task.id.clone(), task)])
}

#[cfg(test)]
#[path = "plan_tests.rs"]
mod tests;
