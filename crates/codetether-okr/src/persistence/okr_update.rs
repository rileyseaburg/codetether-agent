//! Repository okr update operations.

use super::OkrRepository;
use crate::Okr;
use anyhow::Result;
use tokio::fs;
impl OkrRepository {
    /// Update an existing OKR
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
    /// assert!(repo.update_okr(value).await.is_ok());
    /// # });
    /// ```
    pub async fn update_okr(&self, mut okr: Okr) -> Result<Okr> {
        okr.updated_at = chrono::Utc::now();

        // Validate
        okr.validate()?;

        // Write to file
        let path = self.okr_path(okr.id);
        fs::create_dir_all(self.okrs_dir()).await?;
        let json = serde_json::to_string_pretty(&okr)?;
        fs::write(&path, &json).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        if let Some(existing) = cache.okrs.iter_mut().find(|o| o.id == okr.id) {
            *existing = okr.clone();
        }

        tracing::info!(okr_id = %okr.id, "OKR updated");
        Ok(okr)
    }
}
