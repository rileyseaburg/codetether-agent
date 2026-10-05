//! Mandatory goal-maintenance instructions for normal and delegated agents.
//!
//! [`append`] adds the shared turn-boundary contract without creating goals,
//! changing budgets, or mutating the session task log.

const INSTRUCTIONS: &str = include_str!("turn_instructions.md");

/// Append the goal-maintenance contract once to an existing system prompt.
///
/// # Arguments
///
/// * `prompt` — Existing agent instructions, including any goal governance.
///
/// # Returns
///
/// The prompt with the shared contract, preserving all existing instructions.
///
/// # Examples
///
/// An existing persona prompt keeps its instructions before the shared block:
///
/// ```text
/// Existing agent instructions
///
/// ## Session Goal Maintenance — Every Turn
/// ```
pub(crate) fn append(mut prompt: String) -> String {
    if !prompt.contains(INSTRUCTIONS.trim()) {
        prompt.push_str("\n\n");
        prompt.push_str(INSTRUCTIONS);
    }
    prompt
}

#[cfg(test)]
#[path = "turn_instructions_tests.rs"]
mod tests;
