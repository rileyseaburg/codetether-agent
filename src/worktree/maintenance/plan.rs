use super::{WorktreeCleanupReport, plan_support, roots};
use crate::worktree::WorktreeManager;
use anyhow::Result;
use std::path::PathBuf;

impl WorktreeManager {
    /// Preview cleanup of registered worktrees that are clean and merged.
    ///
    /// # Arguments
    ///
    /// * `base` - Branch or commit that candidate worktrees must be merged into.
    /// * `roots` - Optional directory boundaries; an empty slice audits all worktrees.
    ///
    /// # Returns
    ///
    /// A non-mutating report with one safety decision per selected worktree.
    ///
    /// # Errors
    ///
    /// Returns an error when `base` is invalid or Git metadata cannot be read.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::worktree::WorktreeManager;
    ///
    /// let manager = WorktreeManager::for_repo("/srv/project");
    /// let report = manager.plan_merged_cleanup("main", &[]).await?;
    /// assert!(!report.applied);
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn plan_merged_cleanup(
        &self,
        base: &str,
        roots: &[PathBuf],
    ) -> Result<WorktreeCleanupReport> {
        plan_support::verify_base(&self.repo_path, base).await?;
        let current = plan_support::current(&self.repo_path).await?;
        let roots = roots::normalize(&self.repo_path, roots);
        let records = self.registered_worktrees().await?;
        let mut entries = Vec::new();
        for record in records
            .into_iter()
            .filter(|item| roots::includes(item, &roots))
        {
            entries.push(self.inspect_registered(record, base, &current).await);
        }
        Ok(WorktreeCleanupReport {
            base: base.into(),
            applied: false,
            entries,
        })
    }
}
