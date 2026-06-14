//! Async shell execution for the TUI `!command` prefix.

use std::path::Path;
use std::time::{Duration, Instant};

use super::truncate::truncate_output;

/// Outcome of a `!command` shell invocation.
pub(super) struct ShellOutcome {
    pub output: String,
    pub success: bool,
    pub duration_ms: u64,
}

/// Run `command` through `sh -c` in `cwd`, capturing stdout and stderr.
///
/// Output is truncated to 64 KiB and the command is killed after 120s.
pub(super) async fn run_shell(command: &str, cwd: &Path) -> ShellOutcome {
    const TIMEOUT: Duration = Duration::from_secs(120);
    const MAX_OUTPUT: usize = 64 * 1024;
    let started = Instant::now();
    let result = tokio::time::timeout(
        TIMEOUT,
        tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(cwd)
            .output(),
    )
    .await;
    let duration_ms = started.elapsed().as_millis() as u64;
    let (output, success) = match result {
        Ok(Ok(out)) => {
            let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
            let stderr = String::from_utf8_lossy(&out.stderr);
            if !stderr.trim().is_empty() {
                text.push_str(&stderr);
            }
            truncate_output(&mut text, MAX_OUTPUT);
            if text.trim().is_empty() {
                text = "(no output)".to_string();
            }
            (text, out.status.success())
        }
        Ok(Err(error)) => (format!("Failed to spawn shell: {error}"), false),
        Err(_) => ("Command timed out after 120s".to_string(), false),
    };
    ShellOutcome {
        output,
        success,
        duration_ms,
    }
}
