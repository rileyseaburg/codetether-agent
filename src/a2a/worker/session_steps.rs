//! Worker execution adapts approval, output, and task budgets to session control.

use crate::{provider::Provider, session::Session};
use anyhow::Result;
use std::sync::Arc;

mod session_complete;
mod session_context;
#[cfg(test)]
mod session_control_tests;
mod session_failure;
#[cfg(test)]
mod session_failure_tests;
mod session_loop;
mod session_outcome;
mod session_output;
mod session_response;
mod session_step_tools;
mod session_wrap_up;
pub(super) use session_outcome::SessionStepsOutcome;
use session_response::ResponseContext;

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
    crate::session::runtime::boundary::ensure_running(&session.id).await?;
    let mut control = crate::session::runtime::execution_scope::ExecutionScope::open(&session.id)?;
    let context = session_context::Context {
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
    let result = control
        .run(session_loop::run(&context, session, max_steps.max(1)))
        .await;
    if let Err(error) = &result {
        session_failure::record_loop_halt(session, &error.to_string()).await;
    }
    result
}
