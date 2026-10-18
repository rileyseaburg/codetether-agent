//! Public operations against an inherited mux lease authority.

use anyhow::Result;

use std::path::{Path, PathBuf};

/// Environment variable naming the inherited mux session.
pub const SESSION_ENV: &str = "CODETETHER_MUX_SESSION";

/// Request bounded path leases from the inherited mux authority.
/// # Arguments
/// `owner` and `agent` identify the requester; `workspace` anchors `paths`.
/// # Returns
/// The coordinator reply, or None when no mux identity is inherited.
/// # Errors
/// Reports invalid identity, discovery, authentication, transport or protocol failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _reply = codetether_mux_coordination::acquire("turn", "agent", std::path::Path::new("/repo"), vec!["a.rs".into()]).await?;
/// # Ok(()) }
/// ```
pub async fn acquire(
    owner: &str,
    agent: &str,
    workspace: &Path,
    paths: Vec<PathBuf>,
) -> Result<Option<codetether_worktree_lease::CoordinationReply>> {
    super::operation::acquire(owner, agent, workspace, paths).await
}

/// Renew the inherited coordinator's leases for an owner.
/// # Arguments
/// `owner` identifies the active prompt or task.
/// # Returns
/// The coordinator reply, or None without an inherited mux identity.
/// # Errors
/// Propagates discovery, authentication, transport and protocol failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _reply = codetether_mux_coordination::renew("turn").await?;
/// # Ok(()) }
/// ```
pub async fn renew(owner: &str) -> Result<Option<codetether_worktree_lease::CoordinationReply>> {
    super::operation::send(
        "renew",
        owner,
        codetether_worktree_lease::CoordinationRequest::Renew {
            owner: owner.into(),
        },
    )
    .await
}

/// Release the inherited coordinator's leases for an owner.
/// # Arguments
/// `owner` identifies the completed prompt or task.
/// # Returns
/// The coordinator reply, or None without an inherited mux identity.
/// # Errors
/// Propagates discovery, authentication, transport and protocol failures.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _reply = codetether_mux_coordination::release("turn").await?;
/// # Ok(()) }
/// ```
pub async fn release(owner: &str) -> Result<Option<codetether_worktree_lease::CoordinationReply>> {
    super::operation::send(
        "release",
        owner,
        codetether_worktree_lease::CoordinationRequest::Release {
            owner: owner.into(),
        },
    )
    .await
}

/// Test for an inherited mux identity without connecting to a server.
/// # Arguments
/// Reads [SESSION_ENV]; no arguments are needed.
/// # Returns
/// True when the environment variable is present, including an empty value.
/// # Examples
/// ```
/// let _active = codetether_mux_coordination::active();
/// ```
pub fn active() -> bool {
    std::env::var_os(SESSION_ENV).is_some()
}
