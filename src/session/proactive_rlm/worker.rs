//! Native storage binding for the shared proactive worker loop.
mod host;
mod projection;
pub(super) fn spawn(session_id: String) {
    codetether_proactive_worker::spawn(session_id, std::sync::Arc::new(host::NativeHost));
}

pub(crate) async fn prepared_index(
    session: &crate::session::Session,
) -> crate::session::index::SummaryIndex {
    codetether_proactive_context::prepared_index(session, &host::NativeHost).await
}
pub(super) async fn remove(session_id: &str) {
    codetether_proactive_context::remove(session_id, &host::NativeHost).await;
}
