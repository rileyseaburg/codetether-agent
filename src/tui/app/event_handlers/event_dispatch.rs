//! Frontend resource and callback selection for key-event dispatch.
codetether_tui_navigation::declare_frontend_key_dispatch!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crossterm::event::KeyEvent;
    crate::tui::app::event_handlers::event_priority::handle,
    crate::tui::app::event_handlers::keyboard::handle_ctrl_key,
    crate::tui::app::event_handlers::keybinds::handle_unmodified_key;
    crate::tui::app::event_handlers::paste::question_key::handle,
    crate::tui::app::input::goal_answer::editor::handle,
    crate::tui::app::event_handlers::approval_key::scroll,
    crate::tui::app::event_handlers::okr::key::handle;
    crate::tui::app::event_handlers::answer_review_key::handle,
    crate::tui::app::input::sessions::goal_autostart::resume,
);
