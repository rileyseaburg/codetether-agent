//! Workspace and task intent for one ephemeral agent invocation.

use crate::EphemeralPolicy;
use anyhow::Result;
use codetether_agent_input::SpawnRequest;
use codetether_child_workspace::Handoff;
use std::path::PathBuf;

/// Resolve one-shot task intent and allocate isolation for every writable task.
/// # Arguments
/// * `request` — Validated spawn input and optional parent workspace.
/// # Returns
/// Working directory, read-only flag, expected-changes flag and optional checkout.
/// # Errors
/// Propagates managed-checkout allocation errors without shared-write fallback.
/// # Examples
/// Hosts implement [EphemeralPolicy] using the existing task classifier.
pub async fn workspace_policy<H: EphemeralPolicy>(
    request: &SpawnRequest<'_>,
) -> Result<(PathBuf, bool, bool, Option<Handoff>)> {
    let workspace = request
        .parent_workspace
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| ".".into());
    let (read_only, expects_changes) = H::intent(request);
    let handoff = if read_only {
        None
    } else {
        Some(codetether_child_workspace::allocate(&workspace).await?)
    };
    let workspace = handoff
        .as_ref()
        .map_or(workspace, |value| value.workspace.clone());
    Ok((workspace, read_only, expects_changes, handoff))
}
