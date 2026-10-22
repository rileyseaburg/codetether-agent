//! Preserve full pinned sources when acknowledged messages leave memory.
use super::super::state::Checkpoint;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
/// Retain complete sources before acknowledged messages leave memory.
/// # Arguments
/// `state` retains full source records; `messages` and `pages` are aligned;
/// `count` is the prefix being evicted.
/// # Returns
/// Unit after adding any eligible sources to the checkpoint.
/// # Examples
/// ```rust
/// let mut state = codetether_session_store::Checkpoint::default();
/// codetether_session_store::constraints::evicted(&mut state, &[], &[], 0);
/// assert!(state.constraints.is_empty());
/// ```
pub fn evicted(state: &mut Checkpoint, messages: &[Message], pages: &[PageKind], count: usize) {
    for (index, (message, page)) in messages.iter().zip(pages).take(count).enumerate() {
        let turn = state.message_start + index;
        state
            .constraints
            .retain(|record| record.source_turn != turn);
        if let Some(record) = super::ConstraintRecord::from_message(turn, message, *page) {
            state.constraints.push(record);
        }
    }
}
