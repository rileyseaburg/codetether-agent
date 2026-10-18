//! Native session specialization of fail-closed answer-review queries.
/// Read the durable goal state without suppressing journal errors.
pub fn read(session: &str) -> anyhow::Result<crate::tasks::TaskState> {
    codetether_goal_runtime::answer_review::read::<super::super::journal::NativeJournal>(session)
}
/// Check whether an answer-review hold blocks goal continuation.
pub fn held(session: &str) -> bool {
    codetether_goal_runtime::answer_review::held::<super::super::journal::NativeJournal>(session)
}
/// Check whether the held answer is ready for user review.
pub fn ready(session: &str) -> bool {
    codetether_goal_runtime::answer_review::ready::<super::super::journal::NativeJournal>(session)
}
