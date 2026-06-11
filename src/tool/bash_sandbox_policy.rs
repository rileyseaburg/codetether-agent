use std::path::PathBuf;

use crate::config::{Config, SandboxMode};
use crate::tool::sandbox::SandboxPolicy;

pub(super) async fn policy(root: PathBuf, timeout_secs: u64) -> SandboxPolicy {
    SandboxPolicy {
        allowed_paths: writable_paths(root).await,
        allow_network: super::bash_sandbox_config::allow_network(),
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
