//! Native session specialization of a manual goal continuation prompt.
pub(crate) fn prompt(session_id: &str) -> Option<String> {
    codetether_goal_runtime::resume_prompt::<super::journal::NativeJournal>(session_id)
}
