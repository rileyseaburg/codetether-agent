mod discover;
mod git;
mod link;
mod prepare;
mod run;
mod symlink;
#[cfg(test)]
mod tests;

use super::WorktreeManager;
use anyhow::Result;
use std::path::Path;

impl WorktreeManager {
    pub(crate) fn prepare_node_dependencies(&self, worktree: &Path) -> Result<usize> {
        run::execute(&self.repo_path, worktree)
    }
}
