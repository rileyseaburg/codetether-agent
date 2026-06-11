use crate::tool::ToolResult;
use crate::tool::sandbox::SandboxPolicy;
use serde_json::json;
use std::path::Path;

pub(super) fn sandbox_error(
    result: ToolResult,
    policy: &SandboxPolicy,
    cwd: &Path,
    reason: String,
) -> ToolResult {
    result
        .with_metadata("sandboxed", json!(true))
        .with_metadata("sandbox_mode", json!("workspace"))
        .with_metadata("allowed_cwd", json!(cwd.display().to_string()))
        .with_metadata("network_policy", network_policy(policy))
        .with_metadata("unsafe_fallback_reason", json!(reason))
}

fn network_policy(policy: &SandboxPolicy) -> serde_json::Value {
    json!(if policy.allow_network {
        "allowed"
    } else {
        "disabled"
    })
}
