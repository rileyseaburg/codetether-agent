//! Frontend resource and handler selection for modal key priority.
codetether_tui_navigation::declare_frontend_event_priority!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crossterm::event::KeyEvent;
    crate::tui::app::event_handlers::interrupt_key::handle,
    crate::tui::app::event_handlers::interlude_key::handle,
    crate::tui::app::event_handlers::goal_prompt_key::handle_goal_prompt_key,
    crate::tui::app::spawn_form::handle_spawn_form_key,
    crate::tui::app::event_handlers::fuzzy_find_key::handle_fuzzy_find_key,
    crate::tui::app::event_handlers::editor_lsp_key::handle_editor_lsp_key,
    crate::tui::app::event_handlers::editor_key::handle_editor_key,
);
