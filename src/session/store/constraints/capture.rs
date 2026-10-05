//! Keep the bounded constraint prefix when acknowledged messages leave memory.
use super::super::state::Checkpoint;
use crate::provider::Message;
use crate::session::pages::PageKind;
pub(in crate::session::store) fn evicted(
    state: &mut Checkpoint,
    messages: &[Message],
    pages: &[PageKind],
    count: usize,
) {
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
