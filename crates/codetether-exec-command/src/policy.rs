//! Sandbox selection and workspace-write policy for persistent commands.

use codetether_config::{Config, SandboxMode};
use codetether_sandbox_policy::SandboxPolicy;
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
mod enabled;
pub use enabled::enabled;

/// Check whether requested escalation lacks verified invocation authority.
///
/// # Arguments
/// * `args` - Command arguments and approval correlation fields.
/// # Returns
/// Whether the request must be rejected before command planning.
pub fn unapproved_escalation(args: &Value) -> bool {
    escalated(args)
        && !codetether_runtime_policy::approval_gate::approved_or_session_command(
            "exec_command",
            args,
        )
}

fn escalated(args: &Value) -> bool {
    args.get("sandbox_permissions").and_then(Value::as_str) == Some("require_escalated")
}
