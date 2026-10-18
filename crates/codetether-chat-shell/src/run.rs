//! Execute a `!command` through `$SHELL` with a timeout, building a [`ShellEvent`].

use std::path::Path;
use std::time::{Duration, Instant};

use super::event::ShellEvent;
use super::truncate::truncate_output;

const TIMEOUT: Duration = Duration::from_secs(120);
const MAX_OUTPUT: usize = 64 * 1024;

/// Run `command` in `cwd`, capturing stdout/stderr, killing after 120s.
pub(super) async fn run(command: String, cwd: &Path) -> ShellEvent {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let started = Instant::now();
    let result = tokio::time::timeout(
        TIMEOUT,
        tokio::process::Command::new(&shell)
            .arg("-c")
            .arg(&command)
            .current_dir(cwd)
            .output(),
    )
    .await;
    let duration_ms = started.elapsed().as_millis() as u64;
    let (output, success) = interpret(result);
    ShellEvent {
        command,
        output,
        success,
        duration_ms,
    }
}

type SpawnResult = Result<std::io::Result<std::process::Output>, tokio::time::error::Elapsed>;

fn interpret(result: SpawnResult) -> (String, bool) {
    match result {
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
    }
}
