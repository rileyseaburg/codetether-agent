//! Shared workspace allocation without per-worker branches or integration.
use super::SwarmWorktrees;
use crate::worktree_specs::WorktreeSpec;
use codetether_worktree::WorktreeManager;
use std::{path::Path, sync::Arc};

impl SwarmWorktrees {
    /// Assign the same writable workspace to every participant.
    /// # Arguments
    /// * `repo` - Parent workspace, including its current uncommitted edits.
    /// * `tasks` - Tasks in invocation order.
    /// # Returns
    /// Allocation state that performs no branch creation or result merging.
    /// # Examples
    /// ```no_run
    /// let shared = codetether_swarm_tool::worktrees::SwarmWorktrees::shared(
    ///     std::path::Path::new("."), &[]);
    /// assert!(shared.is_shared());
    /// ```
    pub fn shared(repo: &Path, tasks: &[WorktreeSpec]) -> Self {
        Self {
            shared_root: Some(repo.to_path_buf()),
            mgr: Arc::new(WorktreeManager::for_repo(repo).without_vscode_auto_open()),
            infos: tasks.iter().map(|_| Ok(None)).collect(),
            expects_changes: tasks
                .iter()
                .map(|(name, instruction, specialty, preference)| {
                    crate::support_task::build(name, instruction, specialty.as_deref(), *preference)
                        .expects_file_changes()
                })
                .collect(),
            verification: vec![false; tasks.len()],
        }
    }
}
