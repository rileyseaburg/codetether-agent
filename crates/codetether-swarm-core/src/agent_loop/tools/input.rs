//! Sub-agent tool parsing, workspace checks, policy, and provenance enrichment.
use codetether_provenance::{ExecutionOrigin, ExecutionProvenance};
use codetether_runtime_input::enrich_tool_input_with_runtime_context;
use serde_json::Value;
use std::{future::Future, path::Path};

/// Runtime authorization evaluated before authoritative context is attached.
/// # Implementors
/// Native applications retain their session-aware runtime policy evaluator.
pub trait Policy {
    /// Return a denial message, or `None` to continue preparing the invocation.
    fn denial(name: &str, args: &Value) -> impl Future<Output = Option<String>> + Send;
}

/// Parse and prepare a sub-agent tool call under workspace and runtime policy.
/// # Arguments
/// * `working_dir` — Optional isolated workspace root for path validation.
/// * `model` — Current model identifier attached to execution context.
/// * `subtask_id` — Subtask identity used for session and agent provenance.
/// * `name` — Tool being requested.
/// * `raw` — Provider-supplied JSON argument text.
/// # Returns
/// Prepared arguments; malformed JSON retains the existing empty-object fallback.
/// # Errors
/// Returns path-policy or runtime-policy denial before adding execution context.
/// # Examples
/// ```no_run
/// # async fn example<P: codetether_swarm_core::agent_loop::tools::input::Policy>() {
/// let _args = codetether_swarm_core::agent_loop::tools::input::prepare::<P>(None, "model", "task", "read", "{}").await;
/// # }
/// ```
pub async fn prepare<P: Policy>(
    working_dir: Option<&Path>,
    model: &str,
    subtask_id: &str,
    name: &str,
    raw: &str,
) -> Result<Value, String> {
    let mut args = serde_json::from_str(raw).unwrap_or_else(|error| {
        tracing::warn!(target: "codetether_agent::swarm::executor::agent_loop::tool_input", tool = %name, %error, arguments = %raw, "Invalid tool arguments");
        serde_json::json!({})
    });
    if let Some(root) = working_dir {
        codetether_tool_core::path_guard::normalize_tool_args(name, &mut args, root)
            .map_err(|error| format!("Tool path policy denied: {error}"))?;
    }
    if let Some(denial) = P::denial(name, &args).await {
        return Err(denial);
    }
    let agent = format!("agent-{subtask_id}");
    let provenance = ExecutionProvenance::for_operation(&agent, ExecutionOrigin::Swarm);
    Ok(enrich_tool_input_with_runtime_context(
        &args,
        working_dir.unwrap_or_else(|| Path::new(".")),
        Some(model),
        subtask_id,
        &agent,
        Some(&provenance),
    ))
}
