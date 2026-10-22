//! Pre-execution rejection order for normalized tool calls.
use crate::LoopState;
use serde_json::Value;
impl LoopState {
    /// Reject unavailable interactive tools, stub edits, prior declines and repeats.
    /// # Arguments
    /// * `tool`, `input` — Normalized tool identity and JSON arguments.
    /// * `interactive_available` — Deferred frontend capability check.
    /// # Returns
    /// The first rejection reason, or None for an allowed request.
    /// # Examples
    /// ```
    /// let mut state = codetether_prompt_loop::LoopState::new(None);
    /// assert!(state.blocked("question", &serde_json::json!({}), || false).is_some());
    /// ```
    pub fn blocked(
        &mut self,
        tool: &str,
        input: &Value,
        interactive_available: impl FnOnce() -> bool,
    ) -> Option<String> {
        if codetether_prompt_policy::is_interactive_tool(tool) && !interactive_available() {
            return Some("Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".into());
        }
        codetether_edit_policy::detect_stub_in_tool_input(tool, input)
            .or_else(|| self.decline_guard.blocked(tool, input))
    }
}
