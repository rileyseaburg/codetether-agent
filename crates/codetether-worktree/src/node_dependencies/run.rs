use super::{discover, git, link};
use anyhow::Result;
use std::path::Path;

pub(super) fn execute(source: &Path, worktree: &Path) -> Result<usize> {
    let source = source.canonicalize()?;
    let worktree = worktree.canonicalize()?;
    git::verify_same_repository(&source, &worktree)?;
    let mut linked = 0;
    for package in discover::packages(&source)? {
        linked += usize::from(link::package(&source, &worktree, &package)?);
    }
    Ok(linked)
}
