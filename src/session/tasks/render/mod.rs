//! System-prompt rendering for persisted goal governance.

mod goal;
mod status;
mod tasks;

use super::TaskState;

/// Format the current goal and open tasks for model-visible governance.
///
/// # Returns
///
/// `None` when the session has neither a goal nor open tasks.
pub fn governance_block(state: &TaskState) -> Option<String> {
    if state.goal.is_none() && state.open_tasks().is_empty() {
        return None;
    }
    let mut output = String::from("## Goal Governance\n");
    if let Some(current) = &state.goal {
        goal::append(&mut output, current);
    }
    tasks::append(&mut output, state);
    Some(output)
}
