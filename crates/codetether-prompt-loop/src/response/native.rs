//! Retry state for missing promised native tool calls.
use crate::LoopState;
use codetether_prompt_policy::loop_constants as limits;
use codetether_provider_core::ToolDefinition;
impl LoopState {
    /// Increment the native-call retry counter, or reset both counters on calls.
    pub fn native_retry(
        &mut self,
        provider: &str,
        model: &str,
        tools: &[ToolDefinition],
        text: &str,
        calls: bool,
    ) -> bool {
        let retry = codetether_provider_policy::should_retry_missing_native_tool_call(
            provider,
            model,
            self.native_retries,
            tools,
            text,
            calls,
            limits::NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
        );
        if retry {
            self.native_retries += 1;
            return true;
        }
        if calls {
            self.build_retries = 0;
            self.native_retries = 0;
        }
        false
    }
}
