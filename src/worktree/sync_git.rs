use anyhow::{Context, Result};
use std::{path::Path, process::Output};

pub(super) fn git_output(repo_path: &Path, args: &[&str]) -> Result<Output> {
    let mutating = args.first().is_some_and(|name| {
        matches!(
            *name,
            "stash" | "merge" | "reset" | "checkout" | "add" | "commit"
        )
    });
    crate::tool::git::process::output_blocking_refs(repo_path, args, mutating)
        .with_context(|| format!("Failed to execute git {}", args.join(" ")))
}
