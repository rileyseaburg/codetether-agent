//! Repository run list operations.

use super::OkrRepository;
use crate::OkrRun;
use anyhow::Result;
use tokio::fs;
impl OkrRepository {
    /// List all runs
    ///
    /// # Errors
    /// Returns an error if directory entries cannot be read; malformed records are skipped.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(repo.list_runs().await.is_ok());
    /// # });
    /// ```
    pub async fn list_runs(&self) -> Result<Vec<OkrRun>> {
        // Check cache
        {
            let cache = self.cache.read().await;
            if !cache.runs.is_empty() {
                return Ok(cache.runs.clone());
            }
        }

        // Load from disk
        let dir = self.runs_dir();
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut runs = Vec::new();
        let mut entries = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json")
                && let Ok(data) = fs::read_to_string(&path).await
                && let Ok(run) = serde_json::from_str::<OkrRun>(&data)
            {
                runs.push(run);
            }
        }

        // Update cache
        let mut cache = self.cache.write().await;
        cache.runs = runs.clone();

        Ok(runs)
    }
}
