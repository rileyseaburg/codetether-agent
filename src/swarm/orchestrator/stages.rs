//! Dependency-aware stage assignment for decomposed swarm plans.

use crate::swarm::SubTask;
use anyhow::{Result, bail};
use std::collections::HashMap;

pub(super) fn assign(tasks: &mut HashMap<String, SubTask>) -> Result<()> {
    let mut assigned = HashMap::<String, usize>::new();
    while assigned.len() < tasks.len() {
        let ready = ready_tasks(tasks, &assigned);
        if ready.is_empty() {
            bail!("Swarm plan has cyclic or unresolved dependencies");
        }
        for (id, stage) in ready {
            tasks.get_mut(&id).expect("task id came from map").stage = stage;
            assigned.insert(id, stage);
        }
    }
    Ok(())
}

fn ready_tasks(
    tasks: &HashMap<String, SubTask>,
    assigned: &HashMap<String, usize>,
) -> Vec<(String, usize)> {
    tasks
        .iter()
        .filter(|(id, task)| !assigned.contains_key(*id) && dependencies_ready(task, assigned))
        .map(|(id, task)| {
            let stage = task
                .dependencies
                .iter()
                .filter_map(|dependency| assigned.get(dependency))
                .max()
                .map_or(0, |stage| stage + 1);
            (id.clone(), stage)
        })
        .collect()
}

fn dependencies_ready(task: &SubTask, assigned: &HashMap<String, usize>) -> bool {
    task.dependencies.iter().all(|id| assigned.contains_key(id))
}

#[cfg(test)]
#[path = "stages_tests.rs"]
mod tests;
