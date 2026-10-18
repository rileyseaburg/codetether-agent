//! Host adapter for terminal context-budget fallback.
use codetether_provider_core::{Message, ToolDefinition};
use codetether_request_tokens::estimate_request_tokens;

/// Shrink or drop retained history using the application request estimator.
/// # Arguments
/// * `messages` — History whose retained tail may be shrunk or truncated.
/// * `system_prompt` — System text included in the budget estimate.
/// * `tools` — Tool definitions included in the estimate.
/// * `keep_last` — Number of recent messages initially retained.
/// * `target_tokens` — Budget the retained request should fit.
/// # Returns
/// Estimated tokens removed by terminal fallback.
/// # Examples
/// ```
/// let mut messages = Vec::new();
/// assert_eq!(codetether_context_enforcement::terminal_truncate_messages(&mut messages, "", &[], 4, usize::MAX), 0);
/// ```
pub fn terminal_truncate_messages(
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    keep_last: usize,
    target_tokens: usize,
) -> usize {
    codetether_context_policy::compression::terminal::terminal_truncate_messages(
        messages,
        keep_last,
        target_tokens,
        |messages| estimate_request_tokens(system_prompt, messages, tools),
        super::terminal_drop::report_fit,
    )
}
