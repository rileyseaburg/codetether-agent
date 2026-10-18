//! Native recall catalog path resolution.
pub(crate) use codetether_recall_store::paths::canonical;
use std::path::{Path, PathBuf};
pub(crate) fn catalog(workspace: &Path) -> anyhow::Result<PathBuf> {
    codetether_recall_store::paths::catalog(workspace, &super::storage_host::NativeHost)
}
