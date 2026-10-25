//! Compute the net patch a worker branch contributes relative to the target.

use super::git;
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Immutable inputs for one delivery attempt.
pub(super) struct Plan {
    pub root: PathBuf,
    pub branch: String,
    pub head: String,
    pub base: String,
    pub patch: Vec<u8>,
    pub files: Vec<String>,
}

/// Resolve the receiving checkout root containing `target`.
pub(super) fn root(target: &Path) -> Result<PathBuf> {
    Ok(PathBuf::from(git::text(
        target,
        &["rev-parse", "--show-toplevel"],
    )?))
}

/// Compute the patch from `previous` (last delivered head) or the merge base.
///
/// Incremental bases let a long-lived worker deliver repeatedly: only work
/// added since the previous delivery is applied.
pub(super) fn compute(root: &Path, branch: &str, previous: Option<String>) -> Result<Plan> {
    let head = git::text(
        root,
        &["rev-parse", "--verify", &format!("{branch}^{{commit}}")],
    )?;
    let base = match previous.filter(|last| is_ancestor(root, last, &head)) {
        Some(last) => last,
        None => git::text(root, &["merge-base", "HEAD", &head])?,
    };
    let patch = git::bytes(
        root,
        &["diff", "--binary", "--full-index", &base, &head, "--"],
    )?;
    let files = git::text(root, &["diff", "--name-only", &base, &head, "--"])?
        .lines()
        .map(str::to_owned)
        .collect();
    Ok(Plan {
        root: root.to_path_buf(),
        branch: branch.to_string(),
        head,
        base,
        patch,
        files,
    })
}

fn is_ancestor(root: &Path, ancestor: &str, head: &str) -> bool {
    git::run(root, &["merge-base", "--is-ancestor", ancestor, head], None)
        .is_ok_and(|output| output.status.success())
}
