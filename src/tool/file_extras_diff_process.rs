//! Sandboxed process adapter for file and Git diff output.

#[path = "file_extras_diff_path.rs"]
mod path;

use crate::tool::sandbox::{SandboxPolicy, sandbox_output};
use anyhow::Result;
use serde_json::Value;

pub(super) async fn git(invocation: &Value, command: Vec<String>) -> Result<std::process::Output> {
    let root = path::root(invocation)?;
    if let Some(file) = invocation.get("file1").and_then(Value::as_str) {
        path::relative(file)?;
    }
    sandbox_output::run("git", &command, &policy(), &root).await
}

pub(super) async fn files(
    invocation: &Value,
    first: &str,
    second: &str,
) -> Result<std::process::Output> {
    let root = path::root(invocation)?;
    let first = path::confined(&root, first)?;
    let second = path::confined(&root, second)?;
    let args = vec![
        "-u".into(),
        format!("--label={first}"),
        format!("--label={second}"),
        first,
        second,
    ];
    sandbox_output::run("diff", &args, &policy(), &root).await
}

fn policy() -> SandboxPolicy {
    SandboxPolicy {
        allow_exec: true,
        ..Default::default()
    }
}
