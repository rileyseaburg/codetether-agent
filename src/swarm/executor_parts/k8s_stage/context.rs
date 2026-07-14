//! Dependency context assembly for remote subtasks.

use crate::swarm::SubTask;
use std::collections::HashMap;

pub(super) fn build(task: &SubTask, completed: &HashMap<String, String>) -> String {
    let mut output = String::new();
    for id in &task.dependencies {
        if let Some(result) = completed.get(id) {
            output.push_str(&format!(
                "\n--- Result from dependency {id} ---\n{result}\n"
            ));
        }
    }
    output
}
