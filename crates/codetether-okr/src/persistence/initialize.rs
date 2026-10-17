//! Repository initialize operations.

use super::OkrRepository;
use super::state::OkrCache;
use anyhow::{Context, Result};
use std::{path::PathBuf, sync::Arc};
use tokio::{fs, sync::RwLock};
impl OkrRepository {
    /// Create a new OKR repository with the given base path
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let value = OkrRepository::new(std::path::PathBuf::from("example-okrs"));
    /// assert!(value.list_okrs().await.unwrap().is_empty());
    /// # });
    /// ```
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            cache: Arc::new(RwLock::new(OkrCache::default())),
        }
    }

    /// Create repository from Config::data_dir() or fallback to temp
    ///
    /// # Errors
    /// Returns an error if the repository directory cannot be created.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_okr::{OkrRepository};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let result = OkrRepository::from_config().await;
    /// assert!(result.is_ok());
    /// # });
    /// ```
    pub async fn from_config() -> Result<Self> {
        let base_path = codetether_paths::data_dir()
            .map(|p| p.join("okr"))
            .unwrap_or_else(|| std::env::temp_dir().join("codetether").join("okr"));

        // Ensure directory exists
        fs::create_dir_all(&base_path)
            .await
            .context("Failed to create OKR data directory")?;

        tracing::info!(path = %base_path.display(), "OKR repository initialized");

        Ok(Self::new(base_path))
    }
}
