//! Read and persist workspace recall catalogs.

use anyhow::Result;
use std::path::Path;

use super::catalog::RecallCatalog;

/// Read the workspace catalog, defaulting when absent or incompatible.
/// # Arguments
/// Receives a workspace path and native storage host.
/// # Returns
/// A catalog matching the canonical workspace.
/// # Examples
/// ```no_run
/// # async fn example(h: &impl codetether_recall_store::Host) -> anyhow::Result<()> {
/// let catalog = codetether_recall_store::read_catalog(std::path::Path::new("."), h).await;
/// # Ok(())
/// # }
/// ```
pub async fn read<H: crate::Host>(workspace: &Path, host: &H) -> RecallCatalog {
    let canonical = super::paths::canonical(workspace);
    let Ok(path) = super::paths::catalog(&canonical, host) else {
        return RecallCatalog::new(canonical);
    };
    let Some(catalog): Option<RecallCatalog> = super::atomic::read(&path).await else {
        return RecallCatalog::new(canonical);
    };
    if catalog.accepts(&canonical) {
        catalog
    } else {
        RecallCatalog::new(canonical)
    }
}

/// Atomically persist a workspace catalog.
/// # Arguments
/// Receives a catalog and native storage host.
/// # Returns
/// Completes after replacing the catalog file.
/// # Errors
/// Propagates path, serialization and filesystem errors.
/// # Examples
/// ```no_run
/// # async fn example(h: &impl codetether_recall_store::Host, c: &codetether_recall_store::RecallCatalog) -> anyhow::Result<()> {
/// codetether_recall_store::write_catalog(c, h).await
/// # }
/// ```
pub async fn write<H: crate::Host>(catalog: &RecallCatalog, host: &H) -> Result<()> {
    let path = super::paths::catalog(&catalog.workspace, host)?;
    super::atomic::write(&path, catalog).await
}
