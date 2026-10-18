//! Native session specialization of durable goal continuation messages.
pub(crate) async fn next_message(
    session_id: &str,
) -> anyhow::Result<Option<crate::provider::Message>> {
    codetether_goal_runtime::next_message::<super::journal::NativeJournal>(session_id).await
}
