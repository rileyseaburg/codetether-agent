//! Iterative LLM/tool execution loop for worker sessions.

use std::sync::Arc;

use anyhow::Result;

use crate::{provider::Provider, session::Session};

mod session_failure;
#[cfg(test)]
mod session_failure_tests;
mod session_output;
mod session_response;
mod session_step_tools;
use session_failure::{record_loop_halt, step_or_record};
use session_response::{ResponseContext, process_response};

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
    let temperature = Some(if super::prefers_temperature_one(model) {
        1.0
    } else {
        0.7
    });
    let rctx = ResponseContext {
        model,
        tool_registry,
        auto_approve,
        workspace_dir,
        output_callback,
    };
    let mut final_output = String::new();
    let mut completed = false;
    for step in 1..=50 {
        tracing::info!(step, "Agent step starting");
        let response = super::complete_worker_step_with_context_fallback(
            Arc::clone(&provider),
            session,
            model,
            system_prompt,
            tool_definitions,
            temperature,
        )
        .await;
        let response = step_or_record(session, step, response).await?;
        if process_response(&rctx, session, &mut final_output, response).await {
            completed = true;
            break;
        }
    }
    if !completed {
        record_loop_halt(session, "step budget (50) exhausted before completion").await;
    }
    session.save().await?;
    Ok(final_output.trim().to_string())
}
