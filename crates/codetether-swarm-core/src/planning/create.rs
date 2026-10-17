//! Construct tasks and resolve dependency names in two passes.
use super::{wire::DecompositionResponse, *};
pub(super) fn build(parsed: DecompositionResponse) -> Result<Vec<SubTask>> {
    // Create SubTask objects with proper IDs
    let mut subtasks = Vec::new();
    let mut name_to_id: HashMap<String, String> = HashMap::new();

    // First pass: create subtasks and map names to IDs
    for def in &parsed.subtasks {
        let subtask = SubTask::new(&def.name, &def.instruction).with_priority(def.priority);

        let subtask = if let Some(ref specialty) = def.specialty {
            subtask.with_specialty(specialty)
        } else {
            subtask
        };

        let subtask = match def.needs_worktree {
            Some(explicit) => subtask.with_needs_worktree(explicit),
            None => subtask,
        };

        name_to_id.insert(def.name.clone(), subtask.id.clone());
        subtasks.push((subtask, def.dependencies.clone()));
    }

    // Second pass: resolve dependencies
    let result: Result<Vec<SubTask>> = subtasks
        .into_iter()
        .map(|(mut subtask, deps)| {
            subtask.dependencies = dependencies::resolve(&deps, &name_to_id)?;
            Ok(subtask)
        })
        .collect();

    result
}
