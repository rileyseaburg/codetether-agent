//! Frontend type and handler selection for terminal events.
codetether_tui_navigation::declare_frontend_terminal!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle;
    crossterm::event::Event,
    crossterm::event::KeyEvent,
    crossterm::event::MouseEvent;
    crate::tui::app::event_handlers::handle_event,
    crate::tui::app::input::goal_answer::editor::paste,
    crate::tui::app::event_handlers::handle_paste_event,
    crate::tui::app::event_handlers::handle_mouse_event,
);
