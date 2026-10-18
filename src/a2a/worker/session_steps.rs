//! Worker execution adapts approval, output, and task budgets to session control.

use crate::{provider::Provider, session::Session};
use anyhow::Result;
use std::sync::Arc;

use codetether_native_worker::completion::accounted as session_complete;
mod session_context;
#[cfg(test)]
mod session_control_tests;
mod session_failure;
#[cfg(test)]
mod session_failure_tests;
use codetether_a2a_worker_core::session_protocol as session_outcome;
use codetether_a2a_worker_core::session_protocol::transcript as session_output;
mod session_step_tools;
use codetether_a2a_worker_core::session_protocol::context::ResponseContext;
use codetether_a2a_worker_core::session_protocol::transcript as session_wrap_up;
pub(super) use session_outcome::SessionStepsOutcome;

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
    codetether_native_worker::session_loop::run::<
        _,
        _,
        codetether_native_worker::response::Handler<session_step_tools::Runner>,
    >(&context, session, max_steps)
    .await
}
