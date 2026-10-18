//! Append a message while maintaining the persisted working window.
use super::{WorkingSet, before_append};
use codetether_context_policy::pages::{classify, classify_all};
use codetether_provider_core::Message;
/// Repair page alignment, evict persisted prefixes, and append a message.
///
/// # Arguments
/// * `session` — Borrowed transcript, sidecars and persistence checkpoint.
/// * `message` — Message added to the end of the loaded transcript.
/// # Returns
/// Updates vectors and the summary index in place. The caller retains timestamp
/// ownership and can update its session timestamp after this operation returns.
/// # Panics
/// Panics if the persistence checkpoint mutex is poisoned.
/// # Examples
/// ```ignore
/// append(working_set, message);
/// ```
pub fn append<T>(session: WorkingSet<'_, T>, message: Message) {
    if session.pages.len() != session.messages.len() {
        *session.pages = classify_all(&*session.messages).into();
    }
    session.pages.push(classify(&message));
    before_append(WorkingSet {
        id: session.id,
        state: session.state,
        messages: &mut *session.messages,
        pages: &mut *session.pages,
        tools: &mut *session.tools,
        summaries: &mut *session.summaries,
    });
    let appended_idx = session.messages.len();
    session.messages.push(message);
    session.summaries.append(appended_idx);
}
