//! Synchronize session snapshots with transcript views and history boundaries.
use super::View;
use codetether_session_store::Snapshot;

/// Synchronize a session using its own identity as the history source.
/// # Arguments
/// * `app` - Transcript presentation and metric bindings.
/// * `session` - Loaded session snapshot and its durable history state.
/// # Returns
/// Replaces the visible transcript and resets pagination, cache and metrics.
/// # Panics
/// Panics if the session's checkpoint mutex is poisoned.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_chat_transcript::sync::View,
/// # session: &impl codetether_session_store::Snapshot) {
/// codetether_chat_transcript::sync::session::sync_messages_from_session(view, session);
/// # }
/// ```
pub fn sync_messages_from_session(app: &mut impl View, session: &impl Snapshot) {
    sync_messages_from_source(app, session, session.id(), false);
}

/// Synchronize a session whose older history may belong to another source.
/// # Arguments
/// Receives the view, loaded snapshot, source identity and whether the source
/// has older messages beyond the loaded window.
/// # Returns
/// Replaces the visible transcript while preserving its source pagination.
/// # Panics
/// Panics if the session's checkpoint mutex is poisoned.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_chat_transcript::sync::View,
/// # session: &impl codetether_session_store::Snapshot) {
/// codetether_chat_transcript::sync::session::sync_messages_from_source(view, session, "source", true);
/// # }
/// ```
pub fn sync_messages_from_source(
    app: &mut impl View,
    session: &impl Snapshot,
    source_id: &str,
    source_has_older: bool,
) {
    super::sync_messages_from_source(
        app,
        session.messages(),
        session.state().message_count(session.messages().len()),
        source_id,
        source_has_older,
    );
}
