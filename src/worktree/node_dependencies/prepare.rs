use super::WorktreeManager;
use std::path::Path;

impl WorktreeManager {
    pub(crate) fn prepare_validation_dependencies(&self, worktree: &Path) {
        match self.prepare_node_dependencies(worktree) {
            Ok(0) => {}
            Ok(linked) => tracing::info!(linked, worktree = %worktree.display(),
                "Inherited package dependencies for worktree validation"),
            Err(error) => tracing::warn!(%error, worktree = %worktree.display(),
                "Could not inherit package dependencies; validation may need an install"),
        }
    }
}
