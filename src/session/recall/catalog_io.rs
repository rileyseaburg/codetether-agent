//! Native host binding for shared recall catalogs.
use super::storage_host::NativeHost;
use codetether_recall_store::RecallCatalog;
use std::path::Path;
pub(super) async fn read(workspace: &Path) -> RecallCatalog {
    codetether_recall_store::read_catalog(workspace, &NativeHost).await
}
pub(super) async fn write(catalog: &RecallCatalog) -> anyhow::Result<()> {
    codetether_recall_store::write_catalog(catalog, &NativeHost).await
}
