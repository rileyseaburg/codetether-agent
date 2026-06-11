use crate::tool::ToolResult;
use crate::tool::sandbox::{SandboxPolicy, SandboxResult};
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn sandbox_result(
    policy: &SandboxPolicy,
    cwd: &Path,
    result: SandboxResult,
) -> ToolResult {
    let reason = fallback_reason(&result.unsafe_fallbacks);
    ToolResult {
        output: result.output,
        success: result.success,
        metadata: [
            ("exit_code".to_string(), json!(result.exit_code)),
            ("sandboxed".to_string(), json!(true)),
            ("sandbox_mode".to_string(), json!("workspace")),
            ("allowed_cwd".to_string(), json!(cwd.display().to_string())),
            ("network_policy".to_string(), network_policy(policy)),
            (
                "sandbox_violations".to_string(),
                json!(result.sandbox_violations),
            ),
            (
                "unsafe_fallbacks".to_string(),
                json!(result.unsafe_fallbacks),
            ),
            ("unsafe_fallback_reason".to_string(), reason),
        ]
        .into_iter()
        .collect(),
    }
}

fn network_policy(policy: &SandboxPolicy) -> Value {
    json!(if policy.allow_network {
        "allowed"
    } else {
        "disabled"
    })
}

fn fallback_reason(fallbacks: &[String]) -> Value {
    if fallbacks.is_empty() {
        Value::Null
    } else {
        json!(fallbacks.join(", "))
    }
}
