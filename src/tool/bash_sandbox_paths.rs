//! Writable filesystem authority for one sandboxed Bash invocation.

use crate::config::{Config, SandboxMode};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub(super) async fn writable(root: PathBuf, args: &Value) -> Vec<PathBuf> {
    if crate::runtime_policy::approved_or_session_command("bash", args) {
        return vec![root];
    }
    match config(args).await.effective_sandbox_mode() {
        SandboxMode::WorkspaceWrite => vec![root],
        SandboxMode::ReadOnly => Vec::new(),
        SandboxMode::DangerFullAccess => filesystem_root(&root).into_iter().collect(),
    }
}

async fn config(args: &Value) -> Config {
    match crate::tool::network_access::trusted_workspace(args).map(Path::new) {
        Some(workspace) => Config::load_for_workspace(workspace).await,
        None => Config::load().await,
    }
    .unwrap_or_default()
}

fn filesystem_root(root: &Path) -> Option<PathBuf> {
    root.ancestors().last().map(Path::to_path_buf)
}