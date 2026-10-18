//! Application boundaries for planning and the stateful coding worker.
use anyhow::Result;
use codetether_cli_options::RunArgs;
use codetether_validation::ValidationReport;
use std::path::{Path, PathBuf};
/// Provider, worker construction and host-collected diagnostics.
/// # Implementors
/// The application supplies its existing provider and session integration.
#[async_trait::async_trait]
pub trait Host {
    /// Stateful worker retained across implementation and repair turns.
    type Worker: Worker;
    /// Complete a tool-free planning or review request.
    /// # Errors
    /// Returns provider loading, model resolution or completion errors.
    async fn complete(system: String, user: String) -> Result<String>;
    /// Start the coding worker with application session policy.
    /// # Errors
    /// Returns worker setup or model validation errors.
    async fn start(args: &RunArgs, workspace: PathBuf) -> Result<Self::Worker>;
    /// Collect diagnostics from files touched by successful worker tools.
    /// # Errors
    /// Returns diagnostic collection errors.
    async fn collect(workspace: &Path, worker: &Self::Worker) -> Result<Option<ValidationReport>>;
}
/// Stateful worker operations and final result identifiers.
/// # Implementors
/// The application adapts its session-backed coding worker.
#[async_trait::async_trait]
pub trait Worker: Send + Sync {
    /// Current worker model, or an empty string when unset.
    fn model(&self) -> String;
    /// Identity of the worker session.
    fn session_id(&self) -> String;
    /// Execute an implementation or repair prompt.
    /// # Errors
    /// Returns prompt execution or resume errors.
    async fn execute(&mut self, prompt: &str) -> Result<String>;
}
