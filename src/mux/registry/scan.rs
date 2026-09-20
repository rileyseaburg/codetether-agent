//! Tolerant discovery-record scanning and session-name resolution.

use anyhow::{Context, Result};

use super::{MuxRecord, SessionTarget};
#[path = "scan_collect.rs"]
mod collect;

/// Every server record on disk, sorted by key.
pub(in crate::mux) async fn list() -> Result<Vec<MuxRecord>> {
    let root = super::path::root()?;
    tokio::task::spawn_blocking(move || collect::run(&root))
        .await
        .context("scan mux registry")?
}

/// Resolve a globally unique session name to the server hosting it.
pub(in crate::mux) async fn find_session(name: &str) -> Result<Option<SessionTarget>> {
    super::validate_name(name)?;
    Ok(list().await?.into_iter().find_map(|record| {
        record.hosts(name).then(|| SessionTarget {
            record,
            session: name.to_string(),
        })
    }))
}

/// Resolve a session name, failing when no server hosts it.
pub(in crate::mux) async fn load(name: &str) -> Result<SessionTarget> {
    find_session(name)
        .await?
        .with_context(|| format!("mux session '{name}' was not found"))
}

/// The server already bound to `workspace`, if any record exists.
pub(in crate::mux) async fn find_workspace(workspace: &std::path::Path) -> Option<MuxRecord> {
    super::io::load_key(&super::key::for_workspace(workspace))
        .await
        .ok()
}
