//! Native storage capabilities used by recall persistence.
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::PathBuf;
/// Validated paths and projection access for the calling application.
/// # Implementors
/// Implement using the native session store so ID validation stays centralized.
#[async_trait]
pub trait Host: Sync {
    /// Validate the session ID and return its source snapshot path.
    /// # Errors
    /// Propagates invalid IDs and session-directory failures.
    fn session_path(&self, id: &str) -> Result<PathBuf>;
    /// Return the root directory containing recall sidecars and catalogs.
    /// # Errors
    /// Propagates application data-directory resolution failures.
    fn root(&self) -> Result<PathBuf>;
    /// Read the materialized recall projection for a session.
    /// # Errors
    /// Propagates native projection-store failures.
    async fn projection(&self, id: &str) -> Result<Option<Value>>;
}
