//! Writable filesystem authority for one sandboxed command.

use super::approval;
use crate::config::{Config, SandboxMode};
use serde_json::Value;
use std::path::{Path, PathBuf};

pub(super) fn allowed(config: &Config, args: &Value, cwd: &Path) -> Vec<PathBuf> {
    if approval::escalated(args) && approval::exact(args) {
        return filesystem_root(cwd).into_iter().collect();
    }
    if crate::runtime_policy::approved_or_session_command("exec_command", args) {
        return vec![cwd.to_path_buf()];
    }
    match config.effective_sandbox_mode() {
        SandboxMode::WorkspaceWrite => vec![cwd.to_path_buf()],
        SandboxMode::ReadOnly => Vec::new(),
        SandboxMode::DangerFullAccess => filesystem_root(cwd).into_iter().collect(),
    }
}

fn filesystem_root(cwd: &Path) -> Option<PathBuf> {
    cwd.ancestors().last().map(Path::to_path_buf)
}