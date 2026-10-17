//! Repository run queries operations.

use super::OkrRepository;
use crate::{OkrRun, OkrRunStatus};
use anyhow::Result;
use uuid::Uuid;
impl OkrRepository {
    /// Query runs by OKR ID
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_runs_by_okr(uuid::Uuid::nil()).await.is_ok());
    /// # });
    /// ```
    pub async fn query_runs_by_okr(&self, okr_id: Uuid) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all.into_iter().filter(|r| r.okr_id == okr_id).collect())
    }

    /// Query runs by status
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_runs_by_status(codetether_okr::OkrRunStatus::Draft).await.is_ok());
    /// # });
    /// ```
    pub async fn query_runs_by_status(&self, status: OkrRunStatus) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all.into_iter().filter(|r| r.status == status).collect())
    }
}
