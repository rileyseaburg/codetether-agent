//! Bind durable recovery to an authorized local workspace before mutation.
use crate::session::Session;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub(super) enum ScopeError {
    #[error("Authentication required")]
    Unauthenticated,
    #[error("Session workspace access denied")]
    Forbidden,
    #[error("Workspace authority unavailable")]
    Unavailable,
}

/// Compare canonical paths; aliases are accepted, missing paths fail closed.
/// # Errors
/// Returns a scope error for missing, invalid or mismatched directories.
pub(super) fn verify_paths(recorded: &Path, authorized: &Path) -> anyhow::Result<()> {
    let recorded = recorded.canonicalize().map_err(|_| ScopeError::Forbidden)?;
    let authorized = authorized
        .canonicalize()
        .map_err(|_| ScopeError::Forbidden)?;
    if !recorded.is_dir() || recorded != authorized {
        return Err(ScopeError::Forbidden.into());
    }
    Ok(())
}

/// Read only the workspace projection before initializing a recovered session.
/// # Errors
/// Preserves snapshot lookup errors and rejects a different workspace.
pub(super) async fn verify_session(id: &str, authorized: &Path) -> anyhow::Result<PathBuf> {
    let recorded = Session::recorded_workspace(id).await?;
    verify_paths(&recorded, authorized)?;
    Ok(recorded)
}
