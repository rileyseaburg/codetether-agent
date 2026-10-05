//! Read small metadata directly from SQLite without importing legacy projections.
use anyhow::Result;
use std::path::Path;
pub(crate) async fn body(path: &Path) -> Result<Option<String>> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || {
        if !super::marker::is_marker(&path)? {
            return Ok(None);
        }
        let id = super::marker::id(&path)?;
        let db = super::connection::open(&path)?;
        Ok(Some(super::read::header(&db, &id)?.1))
    })
    .await?
}
