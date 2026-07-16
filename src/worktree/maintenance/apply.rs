use super::{WorktreeCleanupReport, WorktreeCleanupState, remove};
use crate::worktree::WorktreeManager;
use anyhow::Result;
use std::path::PathBuf;

impl WorktreeManager {
    /// Remove only registered worktrees proven clean and merged into `base`.
    ///
    /// Local branches are preserved. Dirty, unmerged, locked, current, and
    /// primary checkouts are never removal candidates.
    ///
    /// # Arguments
    ///
    /// * `base` - Branch or commit that candidate worktrees must be merged into.
    /// * `roots` - Optional directory boundaries; an empty slice selects all worktrees.
    ///
    /// # Returns
    ///
    /// A report containing each safety decision and removal outcome.
    ///
    /// # Errors
    ///
    /// Returns an error when planning cannot validate the base or read Git metadata.
    /// Individual removal failures are recorded in the returned report.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::worktree::WorktreeManager;
    /// use std::path::PathBuf;
    ///
    /// let manager = WorktreeManager::for_repo("/srv/project");
    /// let roots = vec![PathBuf::from("/srv/project-worktrees")];
    /// let report = manager.apply_merged_cleanup("main", &roots).await?;
    /// assert!(report.applied);
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn apply_merged_cleanup(
        &self,
        base: &str,
        roots: &[PathBuf],
    ) -> Result<WorktreeCleanupReport> {
        let mut report = self.plan_merged_cleanup(base, roots).await?;
        for entry in &mut report.entries {
            if matches!(
                entry.state,
                WorktreeCleanupState::Ready | WorktreeCleanupState::Prunable
            ) {
                let force = entry.state == WorktreeCleanupState::Prunable;
                match remove::registered(&self.repo_path, &entry.path, force).await {
                    Ok(()) => entry.state = WorktreeCleanupState::Removed,
                    Err(error) => {
                        entry.state = WorktreeCleanupState::RemovalFailed;
                        entry.error = Some(error);
                    }
                }
            }
        }
        report.applied = true;
        Ok(report)
    }
}
