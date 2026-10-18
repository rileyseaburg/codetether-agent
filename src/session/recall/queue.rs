//! Native recall scheduling and session-specialized deletion.
pub(crate) use codetether_native_recall::queue::schedule;
pub(crate) async fn remove(id: &str) -> anyhow::Result<()> {
    codetether_native_recall::queue::remove::<crate::session::Session>(id).await
}
