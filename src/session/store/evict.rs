//! Bound the working set without deleting durable history or forking identity.
use crate::session::Session;
pub(crate) fn before_append(session: &mut Session) {
    let mut state = session.storage.0.lock().unwrap();
    if state.id != session.id || state.revision == 0 {
        return;
    }
    let overflow = session
        .messages
        .len()
        .saturating_sub(super::WINDOW.saturating_sub(1));
    let count = overflow
        .min(session.messages.dirty_from())
        .min(session.pages.dirty_from())
        .min(session.pages.len());
    if count > 0 {
        super::constraints::evicted(&mut state, &session.messages, &session.pages, count);
        session.messages.evict(count);
        session.pages.evict(count);
        state.message_start += count;
        session.summary_index = Default::default();
    }
    let count = session
        .tool_uses
        .len()
        .saturating_sub(super::WINDOW)
        .min(session.tool_uses.dirty_from());
    if count > 0 {
        session.tool_uses.evict(count);
        state.tool_start += count;
    }
}
