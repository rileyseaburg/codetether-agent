//! Tolerant discovery-record scanning and session-name resolution.

use anyhow::{Context, Result};

use super::{MuxRecord, SessionTarget};

/// Every server record on disk, sorted by key.
pub(in crate::mux) async fn list() -> Result<Vec<MuxRecord>> {
    let mut records = Vec::new();
    let Ok(mut entries) = tokio::fs::read_dir(super::path::root()?).await else {
        return Ok(records);
    };
    while let Some(entry) = entries.next_entry().await.context("scan mux registry")? {
        if entry.path().extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let bytes = tokio::fs::read(entry.path())
            .await
            .context("read mux record")?;
        if let Ok(record) = super::io::decode(&bytes) {
            records.push(record);
        }
    }
    records.sort_by(|left, right| left.key.cmp(&right.key));
    Ok(records)
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
