//! Shared `git` subprocess runner for the git tool.

use anyhow::{Context, Result};

/// Run `git <args>` in `cwd` and return combined stdout (and stderr on
/// failure). The boolean reports process success.
///
/// # Errors
///
/// Returns `Err` only if the `git` binary cannot be launched.
pub(super) async fn run_git(cwd: &str, args: &[&str]) -> Result<(String, bool)> {
    let output = tokio::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .await
        .with_context(|| format!("Failed to launch git {args:?}"))?;
    let mut text = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        if !err.trim().is_empty() {
            text = format!("{text}{err}");
        }
    }
    Ok((text, output.status.success()))
}

/// Extract `cwd` from args, defaulting to the current directory.
pub(super) fn cwd_of(args: &serde_json::Value) -> String {
    args.get("cwd")
        .and_then(|v| v.as_str())
        .unwrap_or(".")
        .to_string()
}
