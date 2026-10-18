//! Native session binding for shared transcript synchronization.
mod view;
use crate::{session::Session, tui::app::state::App};
/// Synchronize a session using its own storage identity as history source.
pub fn sync_messages_from_session(app: &mut App, session: &Session) {
    sync_messages_from_source(app, session, &session.id, false);
}
/// Synchronize a loaded session whose older history may live in another source.
pub(crate) fn sync_messages_from_source(
    app: &mut App,
    session: &Session,
    source_id: &str,
    source_has_older: bool,
) {
    codetether_chat_transcript::sync::sync_messages_from_source(
        app,
        session.history(),
        session.message_count(),
        source_id,
        source_has_older,
    );
}
