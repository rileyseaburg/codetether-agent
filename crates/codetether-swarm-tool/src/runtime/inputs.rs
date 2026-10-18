//! Owned values crossing the native direct-swarm execution boundary.
use crate::model_selection::ModelSelection;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::SwarmEvent;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
/// Provider and resolved model chosen for the invocation.
/// # Examples
/// ```no_run
/// # fn selected(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, model: codetether_swarm_tool::model_selection::ModelSelection) {
/// let _selected = codetether_swarm_tool::runtime::SelectedProvider { provider, model };
/// # }
/// ```
pub struct SelectedProvider {
    /// Shared completion provider.
    pub provider: Arc<dyn Provider>,
    /// Requested and resolved model identifiers.
    pub model: ModelSelection,
}
/// Owned setup shared by registry and prompt construction.
/// Constructed by the runtime after successful worktree allocation.
/// # Examples
/// ```no_run
/// # fn context(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _context = codetether_swarm_tool::runtime::WorkerContext {
///     read_only: true, verification: true, expects_changes: false,
///     working_dir: ".".into(), task_id: "review".into(), provider, model: "model".into(),
/// };
/// # }
/// ```
pub struct WorkerContext {
    /// Whether source edits are forbidden.
    pub read_only: bool,
    /// Whether the task verifies existing changes.
    pub verification: bool,
    /// Whether a source patch is required.
    pub expects_changes: bool,
    /// Enforced working directory.
    pub working_dir: PathBuf,
    /// Result-store producer identity.
    pub task_id: String,
    /// Shared completion provider.
    pub provider: Arc<dyn Provider>,
    /// Resolved model identifier.
    pub model: String,
}
/// Prepared inputs for the native provider/tool loop.
/// The host consumes these values without changing their identities or ordering.
/// # Examples
/// ```no_run
/// # fn input<R>(context: codetether_swarm_tool::runtime::WorkerContext, registry: std::sync::Arc<R>, events: tokio::sync::mpsc::Sender<codetether_swarm_core::SwarmEvent>) {
/// let _input = codetether_swarm_tool::runtime::LoopInput {
///     context, registry, events, tools: Vec::new(), system: "Review".into(),
///     user: "Inspect API".into(), max_steps: 50, timeout_secs: 300,
/// };
/// # }
/// ```
pub struct LoopInput<R> {
    /// Registry, provider, model and workspace bindings.
    pub context: WorkerContext,
    /// Repository-aware system prompt.
    pub system: String,
    /// Task-specific user prompt.
    pub user: String,
    /// Definitions from the worker's executable registry.
    pub tools: Vec<ToolDefinition>,
    /// Restricted executable registry.
    pub registry: Arc<R>,
    /// Maximum provider turns.
    pub max_steps: usize,
    /// Worker deadline in seconds.
    pub timeout_secs: u64,
    /// Shared observer event sender.
    pub events: mpsc::Sender<SwarmEvent>,
}
