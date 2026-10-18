//! Build-mode tool-first retry decisions.
use super::BuildContext;
use crate::LoopState;
use codetether_prompt_policy::loop_constants as limits;
use codetether_provider_core::{Message, Role};
impl LoopState {
    /// Decide whether a tool-first retry is needed and increment its counter.
    pub fn tool_first(&mut self, context: BuildContext<'_>, text: &str, calls: bool) -> bool {
        if tool_executed(context.messages) {
            return false;
        }
        let retry = codetether_build_policy::should_force_build_tool_first_retry(
            context.agent,
            self.build_retries,
            context.tools,
            context.messages,
            context.cwd,
            text,
            calls,
            limits::BUILD_MODE_TOOL_FIRST_MAX_RETRIES,
        );
        if retry {
            self.build_retries += 1;
        }
        retry
    }
}

fn tool_executed(messages: &[Message]) -> bool {
    messages
        .iter()
        .rev()
        .take_while(|message| !matches!(message.role, Role::User))
        .any(|message| matches!(message.role, Role::Tool))
}
#[cfg(test)]
#[path = "build_tests.rs"]
mod tests;
