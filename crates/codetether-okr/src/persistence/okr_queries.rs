//! Repository okr queries operations.

use super::OkrRepository;
use crate::{Okr, OkrStatus};
use anyhow::Result;
impl OkrRepository {
    /// Query OKRs by status
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_okrs_by_status(codetether_okr::OkrStatus::Draft).await.is_ok());
    /// # });
    /// ```
    pub async fn query_okrs_by_status(&self, status: OkrStatus) -> Result<Vec<Okr>> {
        let all = self.list_okrs().await?;
        Ok(all.into_iter().filter(|o| o.status == status).collect())
    }

    /// Query OKRs by owner
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_okrs_by_owner("example").await.is_ok());
    /// # });
    /// ```
    pub async fn query_okrs_by_owner(&self, owner: &str) -> Result<Vec<Okr>> {
        let all = self.list_okrs().await?;
        Ok(all
            .into_iter()
            .filter(|o| o.owner.as_deref() == Some(owner))
            .collect())
    }

    /// Query OKRs by tenant
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.query_okrs_by_tenant("example").await.is_ok());
    /// # });
    /// ```
    pub async fn query_okrs_by_tenant(&self, tenant_id: &str) -> Result<Vec<Okr>> {
        let all = self.list_okrs().await?;
        Ok(all
            .into_iter()
            .filter(|o| o.tenant_id.as_deref() == Some(tenant_id))
            .collect())
    }
}
