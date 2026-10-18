//! Frontend type and effect selection for text input.
codetether_tui_navigation::declare_frontend_text_input!(
    crate::tui::app::state::App,
    crate::tui::models::ViewMode,
    crate::tui::models::InputMode,
    crossterm::event::KeyModifiers;
    crate::tui::app::symbols::symbol_search_active,
    crate::tui::app::file_picker::file_picker_filter_push,
    crate::tui::app::file_picker::file_picker_filter_backspace,
    crate::tui::app::symbols::mention::dismiss,
    crate::tui::app::symbols::mention::open,
    crate::tui::app::symbols::schedule_refresh,
);
