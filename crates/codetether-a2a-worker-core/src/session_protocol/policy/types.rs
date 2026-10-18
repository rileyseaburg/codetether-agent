use crate::execution_policy::AutoApprove;
use codetether_provider_core::{Provider, ToolDefinition};
use std::{path::Path, sync::Arc};
/// Optional observer for worker text output.
pub type OutputCallback = Option<Arc<dyn Fn(String) + Send + Sync + 'static>>;
/// Result of a standard non-swarm worker run.
/// # Examples
/// ```
/// let result = codetether_a2a_worker_core::session_protocol::policy::PolicySessionResult {
///     result: codetether_session_events::SessionResult { text: "answer".into(), session_id: "session".into() }, budget_exhausted: false, max_steps: 10,
/// };
/// assert!(!result.budget_exhausted);
/// ```
pub struct PolicySessionResult {
    /// Final text and session identifier.
    pub result: codetether_session_events::SessionResult,
    /// Whether the step budget ended execution.
    pub budget_exhausted: bool,
    /// Configured step budget.
    pub max_steps: usize,
}
/// Borrowed settings passed to native worker-loop execution.
/// # Examples
/// ```no_run
/// # fn configure<T>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, tools: &T) {
/// let request = codetether_a2a_worker_core::session_protocol::policy::RunRequest {
///     provider, model: "model", system_prompt: "instructions", tools, definitions: &[],
///     auto_approve: codetether_a2a_worker_core::execution_policy::AutoApprove::Safe,
///     workspace: std::path::Path::new("."), output: None, max_steps: 10,
/// };
/// assert_eq!(request.max_steps, 10);
/// # }
/// ```
pub struct RunRequest<'a, T> {
    /// Selected provider client.
    pub provider: Arc<dyn Provider>,
    /// Selected model identifier.
    pub model: &'a str,
    /// Prepared system instructions.
    pub system_prompt: &'a str,
    /// Filtered executable registry.
    pub tools: &'a T,
    /// Definitions for the filtered tools.
    pub definitions: &'a [ToolDefinition],
    /// Existing approval policy, unchanged.
    pub auto_approve: AutoApprove,
    /// Effective working directory.
    pub workspace: &'a Path,
    /// Optional output observer.
    pub output: OutputCallback,
    /// Maximum worker steps.
    pub max_steps: usize,
}
