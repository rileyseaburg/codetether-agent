//! Shared constraint indexing with native session-prefix rendering.
use crate::session::Session;
pub(super) use codetether_session_store::constraints::{evicted, read};
pub(crate) fn prefix(session: &Session) -> Vec<String> {
    session
        .storage
        .0
        .lock()
        .unwrap()
        .constraints
        .iter()
        .map(|(seq, text)| format!("- turn {seq}: {text}"))
        .collect()
}
