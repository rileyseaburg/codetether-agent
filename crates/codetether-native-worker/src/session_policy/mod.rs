//! Standard worker execution with native policy and application session state.
//!
//! [`execute`] combines the application's shared session bookkeeping contract
//! with native tool assembly, built-in prompts and the worker step runner.
use codetether_a2a_worker_core::session_protocol::policy;
use codetether_application_registry::{Application, ToolRegistry};
use codetether_native_providers::ProviderRegistry;
use std::marker::PhantomData;
mod execution;
mod source;

struct Native<'a, S, T, W> {
    session: &'a mut S,
    tools: PhantomData<fn() -> (T, W)>,
}

/// Execute standard worker policy for an application session and tool assembly.
/// # Arguments
/// Receives the session, delegated prompt, approval mode, optional model tier,
/// task metadata and output observer without changing their interpretation.
/// # Returns
/// The shared policy result after bounded worker execution.
/// # Errors
/// Propagates provider, title, policy, completion or session persistence errors.
/// # Examples
/// ```no_run
/// # async fn example<S, T, W>(session: &mut S) -> anyhow::Result<()> where S: codetether_native_worker::session_loop::Session + codetether_native_worker::tool_call::InputContext + codetether_a2a_worker_core::session_protocol::policy::Source<Registry = codetether_native_providers::ProviderRegistry>, T: codetether_application_registry::Application, W: codetether_worker_tools::Host<Registry = codetether_application_registry::ToolRegistry<T>> {
/// let _result = codetether_native_worker::session_policy::execute::<S, T, W>(session, "task", codetether_a2a_worker_core::execution_policy::AutoApprove::Safe, None, &serde_json::Map::new(), None).await?;
/// # Ok(()) }
/// ```
pub async fn execute<S, T, W>(
    session: &mut S,
    prompt: &str,
    auto_approve: codetether_a2a_worker_core::execution_policy::AutoApprove,
    model_tier: Option<&str>,
    task_metadata: &serde_json::Map<String, serde_json::Value>,
    output_callback: policy::OutputCallback,
) -> anyhow::Result<policy::PolicySessionResult>
where
    S: crate::session_loop::Session
        + crate::tool_call::InputContext
        + policy::Source<Registry = ProviderRegistry>,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
{
    policy::execute(
        &mut Native::<S, T, W> {
            session,
            tools: PhantomData,
        },
        prompt,
        auto_approve,
        model_tier,
        task_metadata,
        output_callback,
    )
    .await
}
