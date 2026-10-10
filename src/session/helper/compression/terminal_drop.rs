//! Last-resort oldest-message eviction for terminal truncation.

use crate::provider::{Message, Role, ToolDefinition};
use crate::session::helper::token::estimate_request_tokens;

use super::shrink::shrink_retained_payloads_to_budget;

/// Shrink retained payloads, then evict oldest messages, until the request
/// fits `target_tokens` (or nothing more can be removed).
pub(super) fn fit_retained(
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    target_tokens: usize,
) {
    let shrunk = shrink_retained_payloads_to_budget(messages, system_prompt, tools, target_tokens);
    let evicted = drop_oldest_until_fits(messages, system_prompt, tools, target_tokens);
    if shrunk + evicted > 0 {
        tracing::warn!(
            shrunk,
            evicted,
            target_tokens,
            "Terminal truncation fit retained tail"
        );
    }
}

/// Evict the oldest retained messages (after the leading marker at index 0)
/// until the request fits `target_tokens` or only the newest message is
/// left. Orphaned tool results at the new head are evicted too so the
/// provider never sees a result without its call. Returns messages removed.
///
/// This is needed because the active tail anchors on the newest substantive
/// user turn; a long agentic loop after that turn can hold hundreds of
/// messages that payload shrinking alone can never fit into the budget.
pub(super) fn drop_oldest_until_fits(
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    target_tokens: usize,
) -> usize {
    let start = match messages.get(1).map(|m| m.role) {
        Some(Role::User) => 2,
        _ => 1,
    };
    let mut removed = 0;
    while messages.len() > start + 1
        && estimate_request_tokens(system_prompt, messages, tools) > target_tokens
    {
        messages.remove(start);
        removed += 1;
        while messages.len() > start + 1 && matches!(messages[start].role, Role::Tool) {
            messages.remove(start);
            removed += 1;
        }
    }
    removed
}
