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

/// Outcome of a worker session's tool loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SessionStepsOutcome {
    /// Final assistant text (trimmed).
    pub text: String,
    /// `true` when the loop stopped because the step budget ran out before
    /// the agent signalled completion. Callers must not report such a run as
    /// completed work.
    pub budget_exhausted: bool,
    /// Step budget that applied to this run.
    pub max_steps: usize,
}

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
    max_steps: usize,
) -> Result<SessionStepsOutcome> {
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
    let max_steps = max_steps.max(1);
    tracing::info!(max_steps, model, "Worker session step budget");
    for step in 1..=max_steps {
        tracing::info!(step, max_steps, "Agent step starting");
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
        tracing::warn!(max_steps, "Worker session exhausted its step budget before completion");
        record_loop_halt(
            session,
            &format!("step budget ({max_steps}) exhausted before completion"),
        )
        .await;
    }
    session.save().await?;
    Ok(SessionStepsOutcome {
        text: final_output.trim().to_string(),
        budget_exhausted: !completed,
        max_steps,
    })
}
