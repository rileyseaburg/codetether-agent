//! Sandbox selection and workspace-write policy for persistent commands.

use crate::config::Config;
use crate::tool::sandbox::SandboxPolicy;
use serde_json::Value;
use std::path::Path;

#[path = "policy/env.rs"]
pub(crate) mod env;
#[path = "policy/approval.rs"]
mod approval;
#[path = "policy/paths.rs"]
mod paths;
#[path = "policy/workspace.rs"]
mod workspace;

pub(super) async fn resolve(command: &str, args: &Value, cwd: &Path) -> Option<SandboxPolicy> {
    let config = workspace::config(args).await;
    if !enabled(&config, command, args) {
        return None;
    }
    let paths = paths::allowed(&config, args, cwd);
    Some(SandboxPolicy {
        allowed_paths: paths,
        allow_network: crate::tool::network_access::allowed_for(args),
        allow_exec: true,
        timeout_secs: 0,
        ..SandboxPolicy::default()
    })
}

fn enabled(_config: &Config, _command: &str, args: &Value) -> bool {
    let direct = env::truthy("CODETETHER_UNSANDBOXED_BASH")
        || env::is_false("CODETETHER_SANDBOX_BASH");
    if direct && crate::tool::sandbox::direct_fallback_env_allowed() {
        return false;
    }
    let network = crate::tool::network_access::allowed_for(args);
    let unavailable = crate::tool::sandbox::unavailable_reason_for(network).is_some();
    !(unavailable && crate::tool::sandbox::direct_fallback_env_allowed() && approval::exact(args))
}

pub(super) fn direct_disallowed(args: &Value, policy: Option<&SandboxPolicy>) -> bool {
    policy.is_none()
        && (!approval::exact(args) || !crate::tool::network_access::allowed_for(args))
}

pub(super) fn unapproved_escalation(args: &Value) -> bool {
    approval::escalated(args) && !approval::exact(args)
}

#[cfg(test)]
#[path = "policy/tests.rs"]
mod tests;