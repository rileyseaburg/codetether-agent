use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::{commit_msg_hook_script, hook_chmod::make_executable, repo_root};

pub fn install_commit_msg_hook(path: &Path) -> Result<()> {
    let Some(root) = repo_root(path)? else {
        return Ok(());
    };
    let hook_path = root.join(".git/hooks/commit-msg");
    fs::write(&hook_path, commit_msg_hook_script()).context("Failed to write commit-msg hook")?;
    make_executable(&hook_path)?;
    Ok(())
}
