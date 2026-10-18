//! Application-owned durable session operations used during recovery.
use std::path::{Path, PathBuf};
/// Durable session operations; implementations must preserve identity on resume.
/// # Implementors
/// Applications bind their session storage and prompt execution here.
#[async_trait::async_trait]
pub trait RecoverySession: Sized + Send + Sync {
    /// Read the recorded workspace before creating any live runtime state.
    /// # Errors
    /// Returns lookup failures without initializing a session.
    async fn recorded_workspace(id: &str) -> anyhow::Result<PathBuf>;
    /// Resume exactly this identity, returning errors instead of creating one.
    /// # Errors
    /// Returns missing or corrupt session errors without replacement.
    async fn resume(id: &str) -> anyhow::Result<Self>;
    /// Return the loaded session's recorded directory, if present.
    fn directory(&self) -> Option<&Path>;
    /// Return the unchanged durable identity.
    fn id(&self) -> &str;
    /// Apply the requested agent name.
    fn set_agent_name(&mut self, agent: String);
    /// Apply the requested model selector.
    fn set_model(&mut self, model: String);
    /// Persist current metadata and conversation state.
    /// # Errors
    /// Returns durable storage failures.
    async fn save(&self) -> anyhow::Result<()>;
    /// Execute a prompt and return its response text.
    /// # Errors
    /// Returns prompt execution failures for inclusion in the response.
    async fn prompt(&mut self, prompt: &str) -> anyhow::Result<String>;
}
