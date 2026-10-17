//! Repository run update operations.

use super::OkrRepository;
use crate::OkrRun;
use anyhow::Result;
use tokio::fs;
impl OkrRepository {
    /// Update a run
    ///
    /// # Errors
    /// Returns an error if serialization, directory creation, or file writing fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository, OkrRun};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let repo = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// let value = OkrRun::new(uuid::Uuid::nil(), "Run");
    /// assert!(repo.update_run(value).await.is_ok());
    /// # });
    /// ```
    pub async fn update_run(&self, mut run: OkrRun) -> Result<OkrRun> {
        run.updated_at = chrono::Utc::now();

        // Write to file
        let path = self.run_path(run.id);
        fs::create_dir_all(self.runs_dir()).await?;
        let json = serde_json::to_string_pretty(&run)?;
        fs::write(&path, &json).await?;

        // Update cache
        let mut cache = self.cache.write().await;
        if let Some(existing) = cache.runs.iter_mut().find(|r| r.id == run.id) {
            *existing = run.clone();
        }

        tracing::info!(run_id = %run.id, "OKR run updated");
        Ok(run)
    }
}
