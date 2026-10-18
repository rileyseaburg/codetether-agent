//! Frontend resource selection for loop startup.
pub(super) mod mux_status;
codetether_tui_reactor::declare_frontend_loop_setup!(
    crossterm::event::EventStream,
    crate::tui::app::event_loop::setup::mux_status::Reporter,
    crate::tui::constants::MAIN_PROCESSING_WATCHDOG_TIMEOUT_SECS,
    crate::tui::app::signal_shutdown::spawn_shutdown_listener,
);
