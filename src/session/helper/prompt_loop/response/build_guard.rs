//! Guard requiring build agents to call an execution tool first.

use super::super::super::loop_constants as limits;
use super::super::Runner;
use crate::provider::{Message, Role};
use anyhow::Result;

/// Requests a retry when build mode answers without first using tools.
///
/// # Errors
///
/// Reserved for guard failures propagated by the shared response pipeline.
pub(super) fn tool_first(runner: &mut Runner<'_>, text: &str, calls: bool) -> Result<bool> {
    if tool_executed(&runner.session.messages) {
        return Ok(false);
    }
    let retry = super::super::super::build::should_force_build_tool_first_retry(
        &runner.session.agent,
        runner.progress.build_retries,
        &runner.model.tools,
        &runner.session.messages,
        &runner.workspace.cwd,
        text,
        calls,
        limits::BUILD_MODE_TOOL_FIRST_MAX_RETRIES,
    );
    if retry {
        runner.progress.build_retries += 1;
        super::nudge::add(runner, limits::BUILD_MODE_TOOL_FIRST_NUDGE);
    }
    Ok(retry)
}

fn tool_executed(messages: &[Message]) -> bool {
    messages
        .iter()
        .rev()
        .take_while(|message| !matches!(message.role, Role::User))
        .any(|message| matches!(message.role, Role::Tool))
}

#[cfg(test)]
#[path = "build_guard_tests.rs"]
mod tests;
