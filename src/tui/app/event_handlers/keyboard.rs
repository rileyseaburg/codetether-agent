//! Frontend selection for shared modifier shortcuts.
codetether_tui_navigation::declare_frontend_modified_keys!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crossterm::event::KeyEvent,
    crate::tui::models::ViewMode,
    crate::tui::ui::webview::layout_mode::ChatLayoutMode;
    crate::tui::app::event_handlers::side_question::compose_side_question,
    crate::tui::app::event_handlers::copy_reply::handle_copy_reply,
    crate::tui::app::event_handlers::copy_transcript::handle_copy_transcript,
    crate::tui::app::event_handlers::keyboard::handle_voice_key,
    crate::tui::app::event_handlers::clipboard::handle_clipboard_paste,
    crate::tui::app::watchdog::handle_watchdog_cancel;
    crate::tui::app::event_handlers::alt_scroll::handle_alt_scroll,
    crate::tui::app::event_handlers::approval_key::handle,
    crate::tui::app::event_handlers::ctrl_c::handle_ctrl_c,
    crate::tui::app::event_handlers::shared_file_open::ctrl_key,
);
#[cfg(not(test))]
fn handle_voice_key(app: &mut App) {
    super::voice::handle_voice_input(app);
}

#[cfg(test)]
fn handle_voice_key(app: &mut App) {
    app.state.status = "Voice shortcut".to_string();
}
