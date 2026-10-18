//! One bounded verifier-agent execution.
use codetether_provider_core::{Provider, ToolDefinition};
use std::{path::PathBuf, sync::Arc};
/// Prompts, tools and limits passed unchanged to the native agent loop.
/// # Examples
/// ```no_run
/// fn inspect<T>(run: codetether_goal_verification::runtime::Run<'_, T>) {
///     assert_eq!(run.max_steps, 40);
/// }
/// ```
pub struct Run<'a, T> {
    /// Resolved provider.
    pub provider: Arc<dyn Provider>,
    /// Model identifier selected for this provider.
    pub model: &'a str,
    /// System prompt and runtime constraints.
    pub system: &'a str,
    /// Persisted goal and worker evidence.
    pub user: &'a str,
    /// Advertised tool definitions.
    pub definitions: Vec<ToolDefinition>,
    /// Registry backing those definitions.
    pub tools: Arc<T>,
    /// Maximum agent steps.
    pub max_steps: usize,
    /// Maximum wall-clock seconds.
    pub timeout_secs: u64,
    /// Observation identifier used for this run.
    pub id: String,
    /// Workspace enforced for tool calls.
    pub workspace: PathBuf,
}
