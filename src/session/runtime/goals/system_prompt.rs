//! Native session specialization of shared goal-governance prompt rendering.
pub(crate) fn compose(base: &str, session_id: &str) -> String {
    codetether_goal_runtime::compose::<super::journal::NativeJournal>(base, session_id)
}
#[cfg(test)]
#[path = "system_prompt_tests.rs"]
mod tests;
