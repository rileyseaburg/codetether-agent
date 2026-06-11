//! Runtime policy guard for Ralph git mutations.

use anyhow::{Result, bail};
use serde_json::json;
use std::path::Path;
use std::process::{Command, Output};

pub(crate) fn guard(cwd: &Path, command: &str) -> Result<()> {
    let args = json!({ "command": command, "cwd": cwd.display().to_string() });
    let blocked = match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(check(args))),
        Err(_) => runtime_check(args),
    };
    if let Some(blocked) = blocked? {
        bail!("git mutation blocked by policy: {}", blocked.output);
    }
    Ok(())
}

pub(crate) fn checkout(cwd: &Path, branch: &str) -> Result<Output> {
    guard(cwd, &format!("git checkout {branch}"))?;
    Ok(Command::new("git")
        .args(["checkout", branch])
        .current_dir(cwd)
        .output()?)
}

async fn check(args: serde_json::Value) -> Result<Option<crate::tool::ToolResult>> {
    Ok(crate::runtime_policy::evaluate_tool_invocation("bash", &args).await)
}

fn runtime_check(args: serde_json::Value) -> Result<Option<crate::tool::ToolResult>> {
    Ok(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(check(args))?)
}
