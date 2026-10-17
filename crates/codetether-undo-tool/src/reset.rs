//! Execute the existing hard reset after authorization.
use anyhow::Result;
use std::{path::Path, process::Command};

pub(super) fn reset(cwd: &Path, steps: usize) -> Result<std::process::Output> {
    Ok(Command::new("git")
        .args(["reset", "--hard", &format!("HEAD~{steps}")])
        .current_dir(cwd)
        .output()?)
}
