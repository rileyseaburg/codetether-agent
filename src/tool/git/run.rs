//! Shared `git` subprocess runner for the git tool.

use anyhow::{Context, Result};

/// Run `git <args>` in `cwd` and return combined stdout (and stderr on
/// failure). The boolean reports process success.
///
/// # Errors
///
/// Returns `Err` only if the `git` binary cannot be launched.
pub(super) async fn run_git(cwd: &str, args: &[&str]) -> Result<(String, bool)> {
    let cwd = std::path::Path::new(cwd)
        .canonicalize()
        .with_context(|| format!("Invalid git cwd: {cwd}"))?;
    let args = args
        .iter()
        .map(|arg| (*arg).to_string())
        .collect::<Vec<_>>();
    let output = super::process::output(
        &cwd,
        &args,
        &[],
        args.first()
            .is_some_and(|arg| matches!(arg.as_str(), "add" | "commit")),
    )
    .await
    .with_context(|| format!("Failed to launch git {args:?}"))?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok((text, output.status.success()))
}

/// Extract `cwd` from args, defaulting to the current directory.
pub(super) fn cwd_of(args: &serde_json::Value) -> String {
    args.get("cwd")
        .and_then(|v| v.as_str())
        .unwrap_or(".")
        .to_string()
}
