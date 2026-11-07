//! Run one repository hook installer with a bounded timeout.
use anyhow::{Result, anyhow, bail};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;

/// Environment variable overriding the per-installer timeout in seconds.
pub(super) const TIMEOUT_ENV: &str = "CODETETHER_HOOK_INSTALL_TIMEOUT_SECONDS";
const DEFAULT_TIMEOUT_SECONDS: u64 = 900;

/// Per-installer timeout from [`TIMEOUT_ENV`], defaulting to 900 seconds.
pub(super) fn timeout() -> Duration {
    let seconds = std::env::var(TIMEOUT_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|seconds| *seconds > 0)
        .unwrap_or(DEFAULT_TIMEOUT_SECONDS);
    Duration::from_secs(seconds)
}

/// Run `program args...` in `repo_path` with stdin closed.
///
/// Fails when the program cannot start, exits nonzero, or exceeds `limit`.
/// The child is killed when the timeout elapses.
pub(super) async fn run(
    repo_path: &Path,
    program: &str,
    args: &[&str],
    limit: Duration,
) -> Result<()> {
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(repo_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let label = format!("{program} {}", args.join(" "));
    let child = command
        .spawn()
        .map_err(|error| anyhow!("{label} could not start: {error}"))?;
    let output = tokio::time::timeout(limit, child.wait_with_output())
        .await
        .map_err(|_| anyhow!("{label} timed out after {}s", limit.as_secs()))?
        .map_err(|error| anyhow!("{label} failed: {error}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim().lines().last().unwrap_or("no stderr");
        bail!("{label} exited with {}: {detail}", output.status);
    }
    Ok(())
}
