//! Frontend resource selection for per-tick recovery ordering.
codetether_tui_tick::declare_frontend_tick_retry!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::app::session_runtime::TuiSessionHandle;
    crate::tui::app::event_loop::watchdog::check_tick,
    crate::tui::app::event_loop::smart_retry::execute_smart_switch_retry,
);
