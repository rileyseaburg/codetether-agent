use anyhow::{Context, Result};
use std::{
    path::Path,
    process::{Command, Output},
};

pub(super) fn git_output(repo_path: &Path, args: &[&str]) -> Result<Output> {
    Command::new("git")
        .args(args)
        .current_dir(repo_path)
        .output()
        .with_context(|| format!("Failed to execute git {}", args.join(" ")))
}
