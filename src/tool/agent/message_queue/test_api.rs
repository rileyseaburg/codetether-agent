//! Compatibility entrypoints for durable mailbox integration tests.
use super::H;

#[cfg(test)]
pub(crate) async fn claim_for_test(agent_id: &str) -> anyhow::Result<Option<(String, String)>> {
    Ok(codetether_child_mailbox::claim::<H>(agent_id)
        .await?
        .map(|item| (item.id, item.message)))
}

#[cfg(test)]
pub(crate) async fn reset_for_test() {
    codetether_child_mailbox::reset_cache().await
}

#[cfg(test)]
pub(crate) async fn ack_for_test(agent_id: &str, id: &str) -> anyhow::Result<()> {
    codetether_child_mailbox::ack::<H>(agent_id, id).await
}
