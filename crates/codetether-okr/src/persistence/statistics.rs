//! Repository statistics operations.

use super::OkrRepository;
use super::OkrRepositoryStats;
use anyhow::Result;
impl OkrRepository {
    /// Get statistics about the repository
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.stats().await.is_ok());
    /// # });
    /// ```
    pub async fn stats(&self) -> Result<OkrRepositoryStats> {
        let okrs = self.list_okrs().await?;
        let runs = self.list_runs().await?;

        let okr_status_counts = okrs
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, o| {
                *acc.entry(o.status).or_insert(0) += 1;
                acc
            });

        let run_status_counts = runs
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, r| {
                *acc.entry(r.status).or_insert(0) += 1;
                acc
            });

        Ok(OkrRepositoryStats {
            total_okrs: okrs.len(),
            total_runs: runs.len(),
            okr_status_counts,
            run_status_counts,
        })
    }
}
