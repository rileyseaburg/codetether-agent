//! Application services used by mux command execution.
use std::path::PathBuf;

/// Supplies recorded-session lookup and lazy native clipboard access.
///
/// # Implementors
/// Native frontends delegate workspace lookup to their session store and
/// return clipboard text or an encoded image only when paste requests it.
#[async_trait::async_trait]
pub trait Host: Sync {
    /// Resolve the workspace recorded for a durable conversation.
    /// # Errors
    /// Returns invalid-session, storage or missing-workspace errors.
    async fn recorded_workspace(&self, session: &str) -> anyhow::Result<PathBuf>;
    /// Return current clipboard content when it is available.
    fn clipboard_content(&self) -> Option<String>;
}
