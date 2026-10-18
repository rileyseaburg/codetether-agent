use super::ResponseContext;
use codetether_provider_core::{Provider, ToolDefinition};
use std::sync::Arc;
/// Provider request settings and response execution context for a worker run.

/// # Examples
/// ```no_run
/// # fn context(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// use codetether_a2a_worker_core::{execution_policy::AutoApprove, session_protocol::context::{Context, ResponseContext}};
/// let context = Context {
///     provider, model: "model", system_prompt: "instructions", tool_definitions: &[],
///     response: ResponseContext { model: "model", tool_registry: &(), auto_approve: AutoApprove::Safe,
///         workspace_dir: std::path::Path::new("."), output_callback: None },
/// };
/// assert_eq!(context.model, context.response.model);
/// # }
/// ```
pub struct Context<'a, R> {
    /// Provider client used for completion.
    pub provider: Arc<dyn Provider>,
    /// Selected model identifier.
    pub model: &'a str,
    /// Base system instructions for goal-aware composition.
    pub system_prompt: &'a str,
    /// Tool schemas sent with completion requests.
    pub tool_definitions: &'a [ToolDefinition],
    /// Settings used when handling provider responses.
    pub response: ResponseContext<'a, R>,
}
