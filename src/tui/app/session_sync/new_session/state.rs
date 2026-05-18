use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub(super) fn copy_preferences(app: &App, session: &Session, new_session: &mut Session) {
    new_session.metadata.auto_apply_edits = app.state.auto_apply_edits;
    new_session.metadata.allow_network = app.state.allow_network;
    new_session.metadata.slash_autocomplete = app.state.slash_autocomplete;
    new_session.metadata.use_worktree = app.state.use_worktree;
    new_session
        .metadata
        .model
        .clone_from(&session.metadata.model);
}

pub(super) fn reset_chat_state(app: &mut App, session: &Session) {
    app.state.session_id = Some(session.id.clone());
    app.state.messages.clear();
    app.state.streaming_text.clear();
    app.state.processing = false;
    app.state.clear_request_timing();
    app.state.scroll_to_bottom();
    app.state.set_view_mode(ViewMode::Chat);
}
