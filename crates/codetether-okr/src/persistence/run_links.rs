//! Repository run links operations.

use super::OkrRepository;
use crate::OkrRun;
use anyhow::Result;
impl OkrRepository {
    /// Query runs by correlation ID
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_runs_by_correlation("example").await.is_ok());
    /// # });
    /// ```
    pub async fn query_runs_by_correlation(&self, correlation_id: &str) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all
            .into_iter()
            .filter(|r| r.correlation_id.as_deref() == Some(correlation_id))
            .collect())
    }

    /// Query runs by relay checkpoint ID
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_runs_by_checkpoint("example").await.is_ok());
    /// # });
    /// ```
    pub async fn query_runs_by_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all
            .into_iter()
            .filter(|r| r.relay_checkpoint_id.as_deref() == Some(checkpoint_id))
            .collect())
    }

    /// Query runs by session ID
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_runs_by_session("example").await.is_ok());
    /// # });
    /// ```
    pub async fn query_runs_by_session(&self, session_id: &str) -> Result<Vec<OkrRun>> {
        let all = self.list_runs().await?;
        Ok(all
            .into_iter()
            .filter(|r| r.session_id.as_deref() == Some(session_id))
            .collect())
    }
}
