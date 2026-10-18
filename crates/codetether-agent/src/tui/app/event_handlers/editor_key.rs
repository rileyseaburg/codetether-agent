//! Frontend editor modes and external key-action selection.
#[path = "approval_editor/mod.rs"]
mod approval_editor;
codetether_editor::declare_frontend_editor_keys!(
    crate::tui::app::state::App,
    crate::tui::models::ViewMode,
    crossterm::event::KeyEvent;
    crate::tui::app::event_handlers::editor_key::approval_editor::handle,
    crate::tui::app::fuzzy_find::open_fuzzy_find,
);
