use anyhow::{Context, Result, bail};
use std::{path::Path, process::Output};

pub(super) fn output(root: &Path, args: &[&str]) -> Result<Output> {
    crate::tool::git::process::output_blocking_refs(root, args, mutating(args))
        .with_context(|| format!("failed to run git {}", args.join(" ")))
}

fn mutating(args: &[&str]) -> bool {
    args.first().is_some_and(|name| {
        matches!(
            *name,
            "merge" | "cherry-pick" | "checkout" | "reset" | "add" | "commit" | "worktree"
        )
    })
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
