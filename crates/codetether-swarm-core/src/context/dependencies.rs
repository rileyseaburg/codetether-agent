//! Dependency-result context assembly for a subtask.

use crate::SubTask;
use std::collections::HashMap;

/// Render available dependency results in the task's dependency order.
///
/// # Arguments
/// * `task` - Task containing ordered dependency IDs.
/// * `completed` - Results indexed by dependency ID.
///
/// # Returns
/// Formatted context; dependencies without a result are omitted.
///
/// # Examples
/// ```
/// let task = codetether_swarm_core::SubTask::new("inspect", "read source");
/// assert_eq!(codetether_swarm_core::context::dependency_context(&task, &Default::default()), "");
/// ```
pub fn dependency_context(task: &SubTask, completed: &HashMap<String, String>) -> String {
    let mut context = String::new();
    for dependency in &task.dependencies {
        if let Some(result) = completed.get(dependency) {
            context.push_str(&format!(
                "\n--- Result from dependency {dependency} ---\n{result}\n"
            ));
        }
    }
    context
}
