//! Locate the primary checkout without nesting managed child worktrees.

use anyhow::{Context, Result, ensure};
use std::path::{Path, PathBuf};

pub(super) async fn primary(root: &Path) -> Result<PathBuf> {
    let common = super::git::output(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )
    .await?;
    let common = PathBuf::from(common).canonicalize()?;
    let primary = common
        .parent()
        .context("Git common directory has no parent")?;
    // Separate git-dir and bare repositories need an explicit storage policy.
    // Never silently allocate under an arbitrary directory.
    ensure!(
        common.file_name().is_some_and(|name| name == ".git")
            && crate::provenance::repo_root(primary)?.as_deref() == Some(primary),
        "Cannot identify a primary checkout for managed child storage"
    );
    Ok(primary.to_path_buf())
}
