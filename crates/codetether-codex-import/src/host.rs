//! Application boundary for session construction and storage operations.
use anyhow::Result;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
/// Supplies native session operations to the shared import workflow.
/// # Implementors
/// Implement with the application's existing parser and session store.
#[async_trait]
pub trait Host: Send + Sync {
    /// Native session value returned by import and resume.
    type Session: Send;
    /// Parse a rollout, preferring an indexed title when supplied.
    /// # Errors
    /// Propagates parsing failures.
    fn parse(path: &Path, title_override: Option<&str>) -> Result<Self::Session>;
    /// Resolve native session storage.
    /// # Errors
    /// Propagates application directory-resolution failures.
    fn sessions_dir() -> Result<PathBuf>;
    /// Borrow the native session identifier.
    fn session_id(session: &Self::Session) -> &str;
    /// Persist a parsed session at the supplied path.
    /// # Errors
    /// Propagates storage failures.
    async fn persist(session: Self::Session, path: &Path) -> Result<bool>;
    /// Resume the stored session with this identifier.
    /// # Errors
    /// Propagates native resume failures.
    async fn resume(id: &str) -> Result<Self::Session>;
}
