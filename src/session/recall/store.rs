//! Native host binding for shared recall mutation ordering.
use super::storage_host::NativeHost;
use codetether_recall_index::IndexedSession;
pub(super) async fn upsert(indexed: IndexedSession) -> anyhow::Result<()> {
    codetether_recall_store::upsert(indexed, &NativeHost).await
}
pub(super) async fn remove(id: &str) -> anyhow::Result<()> {
    codetether_recall_store::remove(id, &NativeHost).await
}
