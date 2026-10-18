//! Recover a locator lost after the database commit, without inventing a session.
use anyhow::Result;
use std::path::Path;
pub(super) fn run(path: &Path) -> Result<()> {
    if path.exists() {
        return Ok(());
    }
    if !path.parent().unwrap().join("sessions.sqlite3").exists() {
        return Ok(());
    }
    let id = path
        .file_stem()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("invalid session path"))?;
    crate::id::validate(id)?;
    let db = crate::connection::open(path)?;
    if crate::read::contains(&db, id)? {
        crate::marker::create(path, id)?;
    }
    Ok(())
}
