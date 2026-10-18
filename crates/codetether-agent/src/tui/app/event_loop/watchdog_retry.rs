//! Frontend session/runtime selection for watchdog recovery.
codetether_watchdog::declare_frontend_watchdog_retry!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::session::Session,
    crate::tui::app::session_runtime::PromptRequest,
);
