//! Native session specialization of shared goal-governance prompt rendering.
/// Add the current goal governance instructions to a system prompt.
pub fn compose(base: &str, session_id: &str) -> String {
    codetether_goal_runtime::compose::<super::journal::NativeJournal>(base, session_id)
}
#[cfg(test)]
#[path = "system_prompt_tests.rs"]
mod tests;
