use super::*;
/// Increment the turn count, then apply the shared history budget.
///
/// # Arguments
/// * `steps` - Turn counter to increment.
/// * `messages` - Message history to truncate when needed.
///
/// # Returns
/// Nothing; both inputs may be updated.
///
/// # Panics
/// Retains the history-budget arithmetic behavior and counter overflow behavior of the caller.
///
/// # Examples
/// See the module overview for how this operation participates in an agent turn.
pub fn advance_turn(steps: &mut usize, messages: &mut Vec<Message>) {
    *steps += 1;
    crate::context::truncate_messages_to_fit(messages, crate::context::DEFAULT_CONTEXT_LIMIT);
}
