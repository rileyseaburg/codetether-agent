//! Evict oldest retained turns while preserving the marker and user anchor.
use codetether_provider_core::{Message, Role};

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
    target_tokens: usize,
    estimate: &impl Fn(&[Message]) -> usize,
) -> usize {
    let start = match messages.get(1).map(|m| m.role) {
        Some(Role::User) => 2,
        _ => 1,
    };
    let mut removed = 0;
    while messages.len() > start + 1 && estimate(messages) > target_tokens {
        messages.remove(start);
        removed += 1;
        while messages.len() > start + 1 && matches!(messages[start].role, Role::Tool) {
            messages.remove(start);
            removed += 1;
        }
    }
    removed
}
