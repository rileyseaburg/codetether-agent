//! Sandbox selection and workspace-write policy for persistent commands.

use crate::config::{Config, SandboxMode};
use crate::tool::sandbox::SandboxPolicy;
use serde_json::Value;
use std::path::Path;

#[path = "policy/env.rs"]
mod env;

pub(super) async fn resolve(command: &str, args: &Value, cwd: &Path) -> Option<SandboxPolicy> {
    let config = Config::load().await.unwrap_or_default();
    if !enabled(&config, command, args) {
        return None;
    }
    let paths = match config.effective_sandbox_mode() {
        SandboxMode::WorkspaceWrite => vec![cwd.to_path_buf()],
        SandboxMode::ReadOnly | SandboxMode::DangerFullAccess => Vec::new(),
    };
    Some(SandboxPolicy {
        allowed_paths: paths,
        allow_network: env::truthy("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK")
            || env::truthy("CODETETHER_ALLOW_NETWORK"),
        allow_exec: true,
        timeout_secs: 0,
        ..SandboxPolicy::default()
    })
}

fn enabled(config: &Config, command: &str, args: &Value) -> bool {
    if matches!(
        config.effective_sandbox_mode(),
        SandboxMode::DangerFullAccess
    ) || env::truthy("CODETETHER_UNSANDBOXED_BASH")
        || env::is_false("CODETETHER_SANDBOX_BASH")
        || crate::runtime_policy::is_read_only_command(command)
    {
        return false;
    }
    let unavailable = crate::tool::sandbox::unavailable_reason().is_some();
    let approved = crate::runtime_policy::approved_or_session_command("exec_command", args);
    !(unavailable && !crate::tool::sandbox::direct_fallback_env_allowed() && approved)
}
