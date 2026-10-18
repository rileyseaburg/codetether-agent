//! Native session specialization of fail-closed answer-review queries.
pub(crate) fn read(session: &str) -> anyhow::Result<crate::session::tasks::TaskState> {
    codetether_goal_runtime::answer_review::read::<super::super::journal::NativeJournal>(session)
}
pub(crate) fn held(session: &str) -> bool {
    codetether_goal_runtime::answer_review::held::<super::super::journal::NativeJournal>(session)
}
pub(crate) fn ready(session: &str) -> bool {
    codetether_goal_runtime::answer_review::ready::<super::super::journal::NativeJournal>(session)
}
