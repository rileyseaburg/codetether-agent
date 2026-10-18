//! Inputs for one read-only approval reviewer run.
use codetether_provider_core::{Provider, ToolDefinition};
use std::{path::PathBuf, sync::Arc};
/// Resolved reviewer inputs forwarded to the application's agent loop.
/// # Examples
/// ```no_run
/// fn inspect<T>(run: codetether_approval_review::Run<'_, T>) {
///     assert!(run.id.starts_with("review-"));
/// }
/// ```
pub struct Run<'a, T> {
    /// Provider selected from the installed registry.
    pub provider: Arc<dyn Provider>,
    /// Provider-local model identifier.
    pub model: &'a str,
    /// Read-only reviewer system instructions.
    pub system: &'a str,
    /// Proposed change and session-goal context.
    pub user: &'a str,
    /// Advertised read-only tool definitions.
    pub definitions: Vec<ToolDefinition>,
    /// Registry backing the definitions.
    pub tools: Arc<T>,
    /// Maximum loop steps from reviewer configuration.
    pub max_steps: usize,
    /// Maximum loop duration from reviewer configuration.
    pub timeout_secs: u64,
    /// Unique reviewer run identifier.
    pub id: String,
    /// Workspace for read-only inspection.
    pub workspace: PathBuf,
}
