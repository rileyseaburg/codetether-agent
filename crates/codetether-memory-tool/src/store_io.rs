//! store io operations.
use super::{AutoEmbedder, Store};
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;
impl<B: AutoEmbedder> Store<B> {
    /// Get the default memory file path
    /// Resolve the configured memory JSON path.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// The configured data directory plus memory.json, or the legacy fallback.
    /// # Examples
    /// ```rust
    /// let _path = codetether_memory_tool::MemoryStore::default_path();
    /// ```
    pub fn default_path() -> std::path::PathBuf {
        codetether_config::Config::data_dir()
            .map(|p| p.join("memory.json"))
            .unwrap_or_else(|| PathBuf::from(".codetether-agent/memory.json"))
    }

    /// Load from disk
    /// Load entries and refresh their local embeddings.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// The loaded store, or an empty store when no file exists.
    /// # Errors
    /// Propagates serialization and filesystem errors.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let _store = codetether_memory_tool::MemoryStore::load().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load() -> Result<Self> {
        let path = Self::default_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path).await?;
        let mut store: Store<B> = serde_json::from_str(&content)?;
        for entry in store.entries.values_mut() {
            entry.refresh_embedding();
        }
        Ok(store)
    }

    /// Save to disk
    /// Serialize entries to the configured memory JSON path.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Success after creating parent directories and writing the file.
    /// # Errors
    /// Propagates serialization and filesystem errors.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// codetether_memory_tool::MemoryStore::default().save().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn save(&self) -> Result<()> {
        let path = Self::default_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).await?;
        Ok(())
    }
}
