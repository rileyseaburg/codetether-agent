//! Native entry points for shared recall worker lifecycle.
use super::storage_host::NativeHost;
use crate::session::Session;
pub(crate) fn schedule(session: &Session) {
    codetether_recall_worker::schedule(&session.id, std::sync::Arc::new(NativeHost));
}
pub(crate) async fn remove(id: &str) -> anyhow::Result<()> {
    codetether_recall_worker::remove(id, &NativeHost).await
}
