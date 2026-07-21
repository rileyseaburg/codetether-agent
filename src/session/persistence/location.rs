//! Resolve durable sessions independently of the current mux worktree.

mod discovery;
mod registry;

use std::path::PathBuf;

use anyhow::Result;

pub(super) fn resolve(id: &str, local: PathBuf) -> Result<PathBuf> {
    if local.is_file() {
        record(id, &local);
        return Ok(local);
    }
    if let Some(path) = registry::read(id).ok().flatten() {
        return Ok(path);
    }
    let Some(path) = discovery::find(id) else {
        return Ok(local);
    };
    record(id, &path);
    Ok(path)
}

pub(super) fn record(id: &str, path: &std::path::Path) {
    if let Err(error) = registry::write(id, path) {
        tracing::warn!(session_id = id, %error, "session location record failed");
    }
}

pub(super) fn forget(id: &str) {
    if let Err(error) = registry::remove(id) {
        tracing::warn!(session_id = id, %error, "session location cleanup failed");
    }
}
