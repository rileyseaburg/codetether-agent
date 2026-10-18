//! Native storage binding for the shared proactive worker loop.
codetether_proactive_worker::declare_session_host!(
    pub(super) struct NativeHost;,
    crate::session::Session,
    crate::session::store::cursor::next,
    crate::session::store::projection::commit,
    super::store::read_id,
    super::store::write,
    super::store::remove
);
codetether_proactive_context::impl_projection_host!(
    NativeHost,
    crate::session::store::projection::read_session
);
pub(super) fn spawn(session_id: String) {
    codetether_proactive_worker::spawn(session_id, std::sync::Arc::new(NativeHost));
}

pub(crate) async fn prepared_index(
    session: &crate::session::Session,
) -> crate::session::index::SummaryIndex {
    codetether_proactive_context::prepared_index(session, &NativeHost).await
}
pub(super) async fn remove(session_id: &str) {
    codetether_proactive_context::remove(session_id, &NativeHost).await;
}
