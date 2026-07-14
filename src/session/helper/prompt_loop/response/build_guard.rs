//! Guard requiring build agents to call an execution tool first.

use super::super::super::loop_constants as limits;
use super::super::Runner;
use crate::provider::{ContentPart, Message, Role};
use anyhow::Result;

/// Requests a retry when build mode answers without first using tools.
///
/// # Errors
///
/// Reserved for guard failures propagated by the shared response pipeline.
pub(super) fn tool_first(runner: &mut Runner<'_>, text: &str, calls: bool) -> Result<bool> {
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
        nudge(runner, limits::BUILD_MODE_TOOL_FIRST_NUDGE);
    }
    Ok(retry)
}

/// Adds a corrective user-role message to the session transcript.
pub(in crate::session::helper::prompt_loop) fn nudge(runner: &mut Runner<'_>, text: &str) {
    runner.session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    });
}
