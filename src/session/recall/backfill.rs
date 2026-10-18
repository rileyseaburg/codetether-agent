//! Native entry point for shared opt-in recall backfill.
use std::path::Path;
pub(super) fn schedule(workspace: &Path) {
    codetether_recall_worker::schedule_backfill(
        workspace,
        std::sync::Arc::new(super::storage_host::NativeHost),
    );
}
