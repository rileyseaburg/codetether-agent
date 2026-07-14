//! Dependency-result context assembly for a subtask.

use crate::swarm::SubTask;
use std::collections::HashMap;

pub(super) fn build(task: &SubTask, completed: &HashMap<String, String>) -> String {
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
