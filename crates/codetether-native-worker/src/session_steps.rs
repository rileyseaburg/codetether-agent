//! Construct native worker step settings and enter the scoped session loop.
use codetether_a2a_worker_core::{
    execution_policy::AutoApprove,
    session_protocol::{
        SessionStepsOutcome,
        context::{Context, ResponseContext},
    },
};
use codetether_application_registry::{Application, ToolRegistry};
use codetether_native_providers::{Provider, ToolDefinition};
use std::{path::Path, sync::Arc};
/// Run worker steps with the existing provider, tool, approval and output settings.
/// # Arguments
/// Receives the provider/session/model, prompt and tool definitions, execution
/// policy, workspace, optional output callback and maximum step count.
/// # Returns
/// The shared session-loop outcome.
/// # Errors
/// Propagates completion, tool-processing, control or persistence failures.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_native_worker::session_loop::Session + codetether_native_worker::tool_call::InputContext, H: codetether_application_registry::Application>(session: &mut S, provider: std::sync::Arc<dyn codetether_native_providers::Provider>, tools: &codetether_application_registry::ToolRegistry<H>) -> anyhow::Result<()> {
/// let _outcome = codetether_native_worker::run_session_steps(provider, session, "model", "system", tools, &[], codetether_a2a_worker_core::execution_policy::AutoApprove::Safe, std::path::Path::new("."), None, 10).await?;
/// # Ok(()) }
/// ```
pub async fn run_session_steps<
    S: crate::session_loop::Session + crate::tool_call::InputContext,
    H: Application,
>(
    provider: Arc<dyn Provider>,
    session: &mut S,
    model: &str,
    system_prompt: &str,
    tool_registry: &ToolRegistry<H>,
    tool_definitions: &[ToolDefinition],
    auto_approve: AutoApprove,
    workspace_dir: &Path,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    max_steps: usize,
) -> anyhow::Result<SessionStepsOutcome> {
    let context = Context {
        provider,
        model,
        system_prompt,
        tool_definitions,
        response: ResponseContext {
            model,
            tool_registry,
            auto_approve,
            workspace_dir,
            output_callback,
        },
    };
    crate::session_loop::run::<_, _, crate::response::Handler<crate::tool_call::Runner>>(
        &context, session, max_steps,
    )
    .await
}
