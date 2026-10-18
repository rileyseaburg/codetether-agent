//! Exhausted build retry reporting.
use super::BuildContext;
use crate::LoopState;
use codetether_prompt_policy::loop_constants as limits;
impl LoopState {
    /// Reject exhausted retries for a request that requires tool execution.
    /// # Errors
    /// Returns the existing build-mode exhaustion error.
    pub fn ensure_calls(&self, context: BuildContext<'_>) -> anyhow::Result<bool> {
        let build = codetether_build_policy::is_build_agent(context.agent);
        let required =
            codetether_build_policy::build_request_requires_tool(context.messages, context.cwd);
        if build && required && self.build_retries >= limits::BUILD_MODE_TOOL_FIRST_MAX_RETRIES {
            anyhow::bail!(
                "Build mode could not obtain tool calls for an explicit file-change request after {} retries. Switch to a tool-capable model and try again.",
                limits::BUILD_MODE_TOOL_FIRST_MAX_RETRIES
            );
        }
        Ok(false)
    }
}
