use anyhow::Result;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub async fn push_worktree_targets(
    dirs: &mut Vec<PathBuf>,
    mut entries: tokio::fs::ReadDir,
) -> Result<()> {
    while let Some(entry) = entries.next_entry().await? {
        let target = entry.path().join("target");
        if target.is_dir() {
            dirs.push(target);
        }
    }
    Ok(())
}

pub async fn push_existing_children(dirs: &mut Vec<PathBuf>, root: &Path) -> Result<()> {
    let mut entries = match tokio::fs::read_dir(root).await {
        Ok(entries) => entries,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    while let Some(entry) = entries.next_entry().await? {
        if entry.path().is_dir() {
            dirs.push(entry.path());
        }
    }
    Ok(())
}
