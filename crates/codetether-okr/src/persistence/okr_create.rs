//! Repository okr create operations.

use super::OkrRepository;
use crate::Okr;
use anyhow::Result;
use tokio::fs;
impl OkrRepository {
    // ============ OKR CRUD ============

    /// Create a new OKR
    ///
    /// # Errors
    /// Returns an error if validation, serialization, directory access, or file I/O fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository, Okr, KeyResult};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// let mut value = Okr::new("Goal", "Description");
    /// value.add_key_result(KeyResult::new(value.id, "Checks", 10.0, "count"));
    /// assert!(repo.create_okr(value).await.is_ok());
    /// # });
    /// ```
    pub async fn create_okr(&self, okr: Okr) -> Result<Okr> {
        // Validate
        okr.validate()?;

        // Ensure directory exists
        fs::create_dir_all(self.okrs_dir()).await?;

        // Write to file (atomic: tmp + rename)
        let path = self.okr_path(okr.id);
        let tmp_path = path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(&okr)?;
        fs::write(&tmp_path, &json).await?;
        fs::rename(&tmp_path, &path).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.okrs.push(okr.clone());

        tracing::info!(okr_id = %okr.id, title = %okr.title, "OKR created");
        Ok(okr)
    }
}
