//! Host adapter for terminal context-budget fallback.
use crate::provider::{Message, ToolDefinition};
use crate::session::helper::token::estimate_request_tokens;

/// Shrink or drop retained history using the application request estimator.
pub(crate) fn terminal_truncate_messages(
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
