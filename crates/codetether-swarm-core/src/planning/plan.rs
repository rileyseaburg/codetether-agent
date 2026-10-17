//! Validation and fallback construction for swarm plans.

use super::stages;
use crate::SubTask;
use anyhow::Result;
use anyhow::bail;
use std::collections::{HashMap, HashSet};

/// Validate task names and dependency stages, enforcing parallel roots when requested.
///
/// # Arguments
/// * `tasks` - Parsed tasks to index by ID.
/// * `parallel_required` - Whether at least two dependency-free tasks must exist.
///
/// # Returns
/// Tasks indexed by ID with dependency stages assigned.
///
/// # Errors
/// Returns an error for empty/duplicate plans, unresolved or cyclic dependencies, or too few parallel roots.
///
/// # Examples
/// ```
/// let task = codetether_swarm_core::SubTask::new("inspect", "read source");
/// let plan = codetether_swarm_core::planning::validate(vec![task], false).unwrap();
/// assert_eq!(plan.len(), 1);
/// ```
pub fn validate(tasks: Vec<SubTask>, parallel_required: bool) -> Result<HashMap<String, SubTask>> {
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

/// Create the existing single-task fallback plan.
///
/// # Arguments
/// * `instruction` - Instruction for the main task.
///
/// # Returns
/// One main task indexed by its generated ID.
///
/// # Examples
/// ```
/// let plan = codetether_swarm_core::planning::single("inspect source");
/// assert_eq!(plan.len(), 1);
/// ```
pub fn single(instruction: &str) -> HashMap<String, SubTask> {
    let task = SubTask::new("Main Task", instruction);
    HashMap::from([(task.id.clone(), task)])
}

#[cfg(test)]
#[path = "plan_tests.rs"]
mod tests;
