//! Repository okr get operations.

use super::OkrRepository;
use crate::Okr;
use anyhow::{Context, Result};
use tokio::fs;
use uuid::Uuid;
impl OkrRepository {
    /// Get an OKR by ID
    ///
    /// # Errors
    /// Returns an error if an existing file cannot be read or decoded.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.get_okr(uuid::Uuid::nil()).await.is_ok());
    /// # });
    /// ```
    pub async fn get_okr(&self, id: Uuid) -> Result<Option<Okr>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(okr) = cache.okrs.iter().find(|o| o.id == id) {
                return Ok(Some(okr.clone()));
            }
        }

        // Load from disk
        let path = self.okr_path(id);
        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&path).await?;
        let okr: Okr = serde_json::from_str(&data).context("Failed to parse OKR")?;

        // Update cache
        let mut cache = self.cache.write().await;
        if !cache.okrs.iter().any(|o| o.id == okr.id) {
            cache.okrs.push(okr.clone());
        }

        Ok(Some(okr))
    }
}
