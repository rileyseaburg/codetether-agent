//! Canonical workspace boundaries for identity-preserving session recovery.
use std::path::Path;

/// Access failure: unauthenticated caller, forbidden workspace, or unavailable authority.
/// # Examples
/// ```
/// use codetether_session_access::scope::ScopeError;
/// assert_eq!(ScopeError::Forbidden.to_string(), "Session workspace access denied");
/// ```
#[derive(Debug, thiserror::Error)]
pub enum ScopeError {
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
/// # Arguments
/// * `recorded` - Workspace from durable session metadata.
/// * `authorized` - Workspace approved by the control plane.
/// # Returns
/// Unit when canonical directories match.
/// # Examples
/// ```
/// let workspace = tempfile::tempdir().unwrap();
/// codetether_session_access::scope::verify_paths(workspace.path(), workspace.path()).unwrap();
/// ```
pub fn verify_paths(recorded: &Path, authorized: &Path) -> anyhow::Result<()> {
    let recorded = recorded.canonicalize().map_err(|_| ScopeError::Forbidden)?;
    let authorized = authorized
        .canonicalize()
        .map_err(|_| ScopeError::Forbidden)?;
    if !recorded.is_dir() || recorded != authorized {
        return Err(ScopeError::Forbidden.into());
    }
    Ok(())
}
