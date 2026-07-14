use anyhow::{Context, Result, bail};
use std::{
    path::Path,
    process::{Command, Output},
};

pub(super) fn output(root: &Path, args: &[&str]) -> Result<Output> {
    Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .with_context(|| format!("failed to run git {}", args.join(" ")))
}

pub(super) fn text(root: &Path, args: &[&str]) -> Result<String> {
    let output = output(root, args)?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub(super) fn lines(root: &Path, args: &[&str]) -> Result<Vec<String>> {
    Ok(text(root, args)?
        .lines()
        .map(str::to_owned)
        .filter(|line| !line.is_empty())
        .collect())
}
