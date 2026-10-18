//! Frontend detector, timeout and runtime selection for watchdog checks.
codetether_watchdog::declare_frontend_watchdog!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::tui::constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
    crate::tui::app::watchdog::check_watchdog_stall,
);
