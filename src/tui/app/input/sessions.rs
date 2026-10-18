//! Frontend session filter and selection bindings.
#[path = "goal_autostart.rs"]
pub(crate) mod goal_autostart;
codetether_session_picker::declare_frontend_session_input!(
    crate::tui::app::state::App,
    crossterm::event::KeyModifiers,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle;
    crate::tui::app::input::continue_command::select_session,
);
