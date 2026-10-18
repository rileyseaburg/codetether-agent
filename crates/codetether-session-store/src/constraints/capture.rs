//! Keep the bounded constraint prefix when acknowledged messages leave memory.
use super::super::state::Checkpoint;
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;
/// Retain protected excerpts before acknowledged messages leave memory.
/// # Arguments
/// `state` retains at most eight excerpts; `messages` and `pages` are aligned;
/// `count` is the prefix being evicted.
/// # Returns
/// Unit after adding any eligible excerpts to the checkpoint.
/// # Examples
/// ```rust
/// let mut state = codetether_session_store::Checkpoint::default();
/// codetether_session_store::constraints::evicted(&mut state, &[], &[], 0);
/// assert!(state.constraints.is_empty());
/// ```
pub fn evicted(state: &mut Checkpoint, messages: &[Message], pages: &[PageKind], count: usize) {
    for (index, (message, page)) in messages.iter().zip(pages).take(count).enumerate() {
        if state.constraints.len() == 8 {
            break;
        }
        if *page == PageKind::Constraint
            && let Some(text) = super::excerpt(message)
        {
            state.constraints.push((state.message_start + index, text));
        }
    }
}
