//! Blocking Git subprocesses with optional stdin for patch application.

use anyhow::{Context, Result, bail};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub(super) fn run(cwd: &Path, args: &[&str], stdin: Option<&[u8]>) -> Result<Output> {
    let mut child = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to run git {}", args.join(" ")))?;
    if let Some(input) = stdin {
        let mut pipe = child.stdin.take().context("git stdin unavailable")?;
        pipe.write_all(input)?;
    }
    child
        .wait_with_output()
        .with_context(|| format!("failed to wait for git {}", args.join(" ")))
}

pub(super) fn bytes(cwd: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = run(cwd, args, None)?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(output.stdout)
}

pub(super) fn text(cwd: &Path, args: &[&str]) -> Result<String> {
    Ok(String::from_utf8(bytes(cwd, args)?)?.trim().to_string())
}
