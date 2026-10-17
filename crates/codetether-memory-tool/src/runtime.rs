//! Memory tool state and disk initialization lifecycle.
use super::{AutoEmbedder, Store};
use anyhow::Result;

/// Store and retrieve persistent knowledge through tool actions.
///
/// # Examples
/// ```rust
/// let _tool = codetether_memory_tool::MemoryTool::new();
/// ```
pub struct Runtime<B> {
    pub(crate) store: tokio::sync::Mutex<Store<B>>,
    pub(crate) initialized: std::sync::atomic::AtomicBool,
}
impl<B: AutoEmbedder> Runtime<B> {
    /// Create an uninitialized tool with an empty in-memory store.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool that loads its persisted data on first use.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_memory_tool::MemoryTool::new();
    /// ```
    pub fn new() -> Self {
        Self {
            store: tokio::sync::Mutex::new(Store::<B>::default()),
            initialized: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Initialize store from disk (once).
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Success after the initialization attempt.
    /// # Errors
    /// Load errors are intentionally ignored, preserving the current store.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// codetether_memory_tool::MemoryTool::new().init().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn init(&self) -> Result<()> {
        use std::sync::atomic::Ordering;

        if self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }

        let mut store = self.store.lock().await;
        if let Ok(loaded) = Store::<B>::load().await {
            *store = loaded;
        }
        self.initialized.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Persist store to disk.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Success after serialization and writing.
    /// # Errors
    /// Propagates serialization and filesystem failures.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// codetether_memory_tool::MemoryTool::new().persist().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn persist(&self) -> Result<()> {
        let store = self.store.lock().await;
        store.save().await
    }
}
impl<B: AutoEmbedder> Default for Runtime<B> {
    fn default() -> Self {
        Self::new()
    }
}
