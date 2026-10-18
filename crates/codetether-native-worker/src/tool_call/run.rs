//! Shared worker tool-call coordination over native registry services.
use super::*;
/// Enrich and execute one worker tool call under the existing approval policy.
/// # Arguments
/// Receives the session, registry, policy, workspace/model, output observer and
/// original provider tool-call tuple.
/// # Returns
/// Completes after the tool reply has been recorded on the session transcript.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_native_worker::tool_call::InputContext, H: codetether_application_registry::Application>(session: &mut S, registry: &codetether_application_registry::ToolRegistry<H>) {
/// codetether_native_worker::tool_call::execute_tool_call(session, registry, codetether_a2a_worker_core::execution_policy::AutoApprove::None, std::path::Path::new("."), "model", &None, ("call".into(), "read".into(), serde_json::json!({}))).await;
/// # }
/// ```
pub async fn execute_tool_call<S: InputContext, H: Application>(
    session: &mut S,
    registry: &ToolRegistry<H>,
    auto_approve: AutoApprove,
    workspace_dir: &Path,
    model: &str,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    (tool_id, tool_name, tool_input): ToolCall,
) {
    codetether_a2a_worker_core::session_protocol::tool_call::execute_tool_call(
        &mut host::ApplicationHost {
            session,
            registry,
            workspace_dir,
            model,
            cb,
        },
        auto_approve,
        cb,
        (tool_id, tool_name, tool_input),
    )
    .await;
}
