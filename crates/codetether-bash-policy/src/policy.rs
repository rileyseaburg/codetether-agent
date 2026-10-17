//! Build the effective bash sandbox policy from shared configuration.
//! `policy` applies workspace write access, network settings, and a timeout.

use std::path::PathBuf;

use codetether_config::{Config, SandboxMode};
use codetether_sandbox_policy::SandboxPolicy;

/// Build a bash sandbox policy using the current configuration.
///
/// # Arguments
/// `root` is the workspace directory; `timeout_secs` limits execution time.
/// # Returns
/// The configured writable paths, network access, and execution limits.
///
/// # Examples
/// ```rust
/// let _policy = codetether_bash_policy::policy::policy("/workspace".into(), 30);
/// ```
pub async fn policy(root: PathBuf, timeout_secs: u64) -> SandboxPolicy {
    SandboxPolicy {
        allowed_paths: writable_paths(root).await,
        allow_network: super::config::allow_network(),
        allow_exec: true,
        timeout_secs,
        ..SandboxPolicy::default()
    }
}

async fn writable_paths(root: PathBuf) -> Vec<PathBuf> {
    match Config::load()
        .await
        .unwrap_or_default()
        .effective_sandbox_mode()
    {
        SandboxMode::WorkspaceWrite => vec![root],
        SandboxMode::ReadOnly | SandboxMode::DangerFullAccess => Vec::new(),
    }
}
