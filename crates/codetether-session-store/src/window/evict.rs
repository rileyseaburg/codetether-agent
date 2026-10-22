//! Bound the working set without deleting durable history or forking identity.
use super::{WINDOW, WorkingSet};
/// Evict only persisted leading records before appending another message.
/// # Arguments
/// `session` contains separate references to native vectors and checkpoint state.
/// # Returns
/// Unit; loaded offsets advance without deleting durable records.
/// # Panics
/// Panics if the checkpoint mutex is poisoned.
/// # Examples
/// See [WorkingSet] for constructing the borrowed field view.
pub fn before_append<T>(session: WorkingSet<'_, T>) {
    let mut state = session.state.0.lock().unwrap();
    if state.id != session.id || state.revision == 0 {
        return;
    }
    let overflow = session
        .messages
        .len()
        .saturating_sub(WINDOW.saturating_sub(1));
    let count = overflow
        .min(session.messages.dirty_from())
        .min(session.pages.dirty_from())
        .min(session.pages.len());
    if count > 0 {
        crate::constraints::evicted(&mut state, &*session.messages, &*session.pages, count);
        session.messages.evict(count);
        session.pages.evict(count);
        state.message_start += count;
        session.summaries.evict_prefix(count);
    }
    let count = session
        .tools
        .len()
        .saturating_sub(WINDOW)
        .min(session.tools.dirty_from());
    if count > 0 {
        session.tools.evict(count);
        state.tool_start += count;
    }
}
