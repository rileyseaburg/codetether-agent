//! Workspace verification and recovery of an existing durable identity.
use crate::execution;
use crate::{RecoverySession, ResumeRequest};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// Resume only the requested durable identity, preserving all storage errors.
/// # Errors
/// Returns workspace-scope and durable storage failures without creating a session.
/// # Arguments
/// * `id` - Existing durable session identifier.
/// * `request` - Metadata overrides and optional prompt.
/// * `workspace` - Authorized workspace directory.
/// # Returns
/// Ready, completed or failed response for the same session identity.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_recovery::RecoverySession>(id: &str, workspace: &std::path::Path) {
/// let request = codetether_session_recovery::ResumeRequest { prompt: None, agent: None, model: None };
/// let _ = codetether_session_recovery::resume::<S>(id, request, workspace).await;
/// # }
/// ```
pub async fn resume<S: RecoverySession>(
    id: &str,
    request: ResumeRequest,
    workspace: &std::path::Path,
) -> anyhow::Result<Value> {
    let recorded = verify_session::<S>(id, workspace).await?;
    let mut session = S::resume(id).await?;
    codetether_session_access::scope::verify_paths(
        session.directory().unwrap_or(&recorded),
        workspace,
    )?;
    if let Some(agent) = request.agent {
        session.set_agent_name(agent);
    }
    if let Some(model) = request.model {
        session.set_model(model);
    }
    execution::finish(session, request.prompt).await
}

/// Read only the workspace projection before initializing a recovered session.
/// # Errors
/// Preserves snapshot lookup errors and rejects a different workspace.
async fn verify_session<S: RecoverySession>(
    id: &str,
    authorized: &Path,
) -> anyhow::Result<PathBuf> {
    let recorded = S::recorded_workspace(id).await?;
    codetether_session_access::scope::verify_paths(&recorded, authorized)?;
    Ok(recorded)
}
