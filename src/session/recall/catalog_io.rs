//! Read and persist workspace recall catalogs.

use anyhow::Result;
use std::path::Path;

use super::catalog::RecallCatalog;

pub(super) async fn read(workspace: &Path) -> RecallCatalog {
    let canonical = super::paths::canonical(workspace);
    let Ok(path) = super::paths::catalog(&canonical) else {
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

pub(super) async fn write(catalog: &RecallCatalog) -> Result<()> {
    let path = super::paths::catalog(&catalog.workspace)?;
    super::atomic::write(&path, catalog).await
}
