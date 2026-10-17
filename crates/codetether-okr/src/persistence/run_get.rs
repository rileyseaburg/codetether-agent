//! Repository run get operations.

use super::OkrRepository;
use crate::OkrRun;
use anyhow::{Context, Result};
use tokio::fs;
use uuid::Uuid;
impl OkrRepository {
    /// Get a run by ID
    ///
    /// # Errors
    /// Returns an error if an existing file cannot be read or decoded.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.get_run(uuid::Uuid::nil()).await.is_ok());
    /// # });
    /// ```
    pub async fn get_run(&self, id: Uuid) -> Result<Option<OkrRun>> {
        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some(run) = cache.runs.iter().find(|r| r.id == id) {
                return Ok(Some(run.clone()));
            }
        }

        // Load from disk
        let path = self.run_path(id);
        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&path).await?;
        let run: OkrRun = serde_json::from_str(&data).context("Failed to parse OKR run")?;

        // Update cache
        let mut cache = self.cache.write().await;
        if !cache.runs.iter().any(|r| r.id == run.id) {
            cache.runs.push(run.clone());
        }

        Ok(Some(run))
    }
}
