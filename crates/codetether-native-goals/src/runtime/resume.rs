//! Native session specialization of a manual goal continuation prompt.
/// Render the manual continuation prompt for the active session goal.
pub fn prompt(session_id: &str) -> Option<String> {
    codetether_goal_runtime::resume_prompt::<super::journal::NativeJournal>(session_id)
}
