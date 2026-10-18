//! Host contract for loading the initial run session.
use std::path::Path;
/// Native session import, resume and creation operations.
/// # Implementors
/// Applications delegate to their durable session store and import adapter.
#[async_trait::async_trait]
pub trait SessionSource: Sized + Send {
    /// Identity used by selection logging.
    fn id(&self) -> &str;
    /// Create a new session.
    /// # Errors
    /// Returns native creation errors.
    async fn create() -> anyhow::Result<Self>;
    /// Import a Codex session by its identity.
    /// # Errors
    /// Returns native import errors.
    async fn import_codex(id: &str) -> anyhow::Result<Self>;
    /// Resume an explicitly selected session.
    /// # Errors
    /// Returns native lookup or loading errors.
    async fn resume(id: &str) -> anyhow::Result<Self>;
    /// Resume the latest session in the workspace scope.
    /// # Errors
    /// Returns native lookup or loading errors.
    async fn resume_last(workspace: Option<&Path>) -> anyhow::Result<Self>;
}
