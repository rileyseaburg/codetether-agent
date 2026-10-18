//! Native host binding for shared recall sidecar I/O.
use super::storage_host::NativeHost;
use codetether_recall_index::IndexedSession;
pub(super) async fn read(id: &str) -> Option<IndexedSession> {
    codetether_recall_store::read(id, &NativeHost).await
}
