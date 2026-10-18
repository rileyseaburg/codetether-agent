//! Argument translation for MCP `run_command`.

use anyhow::Result;
use serde_json::{Value, json};

const PRESERVE: &[&str] = &[
    "approval_id",
    "prefix_rule",
    "workspace",
    "session_id",
    "agent",
];

pub(super) fn translate(args: &Value) -> Result<Value> {
    let command = args
        .get("command")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("Missing command"))?;
    let mut bash_args = json!({ "command": command, "timeout": timeout_secs(args) });
    if let Some(cwd) = args.get("cwd").and_then(Value::as_str) {
        bash_args["cwd"] = json!(cwd);
    }
    preserve(args, &mut bash_args);
    Ok(bash_args)
}

fn preserve(args: &Value, bash_args: &mut Value) {
    for key in PRESERVE {
        if let Some(value) = args.get(key) {
            bash_args[*key] = value.clone();
        }
    }
}

fn timeout_secs(args: &Value) -> u64 {
    args.get("timeout_ms")
        .and_then(Value::as_u64)
        .map(|ms| ms.div_ceil(1000).max(1))
        .unwrap_or(30)
}
