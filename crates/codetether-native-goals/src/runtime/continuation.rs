//! Native session specialization of durable goal continuation messages.
/// Build the next durable goal continuation message, if one is needed.
pub async fn next_message(
    session_id: &str,
) -> anyhow::Result<Option<codetether_provider_core::Message>> {
    codetether_goal_runtime::next_message::<super::journal::NativeJournal>(session_id).await
}
