//! Repository run create operations.

use super::OkrRepository;
use crate::OkrRun;
use anyhow::Result;
use tokio::fs;
impl OkrRepository {
    // ============ OKR Run CRUD ============

    /// Create a new OKR run
    ///
    /// # Errors
    /// Returns an error if validation, serialization, directory access, or file I/O fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository, OkrRun};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// let value = OkrRun::new(uuid::Uuid::nil(), "Run");
    /// assert!(repo.create_run(value).await.is_ok());
    /// # });
    /// ```
    pub async fn create_run(&self, run: OkrRun) -> Result<OkrRun> {
        // Validate
        run.validate()?;

        // Ensure directory exists
        fs::create_dir_all(self.runs_dir()).await?;

        // Write to file
        let path = self.run_path(run.id);
        let json = serde_json::to_string_pretty(&run)?;
        fs::write(&path, &json).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        cache.runs.push(run.clone());

        tracing::info!(run_id = %run.id, okr_id = %run.okr_id, name = %run.name, "OKR run created");
        Ok(run)
    }
}
