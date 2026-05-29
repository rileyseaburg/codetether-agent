//! Iterative LLM/tool execution loop for worker sessions.

use std::sync::Arc;

use anyhow::Result;

use crate::{provider::Provider, session::Session};

mod session_step_tools;
use session_step_tools::{append_text_output, collect_tool_calls, execute_tool_call};

pub(super) async fn run_session_steps(
    provider: Arc<dyn Provider>,
    session: &mut Session,
    model: &str,
    system_prompt: &str,
    tool_registry: &crate::tool::ToolRegistry,
    tool_definitions: &[crate::provider::ToolDefinition],
    auto_approve: super::AutoApprove,
    workspace_dir: &std::path::Path,
    output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> Result<String> {
    let temperature = if super::prefers_temperature_one(model) { Some(1.0) } else { Some(0.7) };
    let mut final_output = String::new();
    for step in 1..=50 {
        tracing::info!(step, "Agent step starting");
        let response = super::complete_worker_step_with_context_fallback(
            Arc::clone(&provider), session, model, system_prompt, tool_definitions, temperature,
        ).await?;
        crate::telemetry::TOKEN_USAGE.record_model_usage(model,
            response.usage.prompt_tokens as u64, response.usage.completion_tokens as u64);
        let tool_calls = collect_tool_calls(&response.message.content);
        append_text_output(&response.message.content, &mut final_output, &output_callback);
        session.add_message(response.message.clone());
        if tool_calls.is_empty() { break; }
        for tool_call in tool_calls {
            execute_tool_call(session, tool_registry, auto_approve, workspace_dir, model, &output_callback, tool_call).await;
        }
    }
    session.save().await?;
    Ok(final_output.trim().to_string())
}
