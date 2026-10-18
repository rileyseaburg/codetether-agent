//! Native session lookup before shared workspace validation.
use crate::session::Session;
pub(super) use codetether_session_access::scope::{ScopeError, verify_paths};
use std::path::{Path, PathBuf};

/// Read only the workspace projection before initializing a recovered session.
/// # Errors
/// Preserves snapshot lookup errors and rejects a different workspace.
pub(super) async fn verify_session(id: &str, authorized: &Path) -> anyhow::Result<PathBuf> {
    let recorded = Session::recorded_workspace(id).await?;
    verify_paths(&recorded, authorized)?;
    Ok(recorded)
}
