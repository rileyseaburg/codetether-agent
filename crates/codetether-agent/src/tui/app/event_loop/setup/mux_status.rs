//! Frontend type selection for shared mux status reporting.
mod snapshot;
codetether_mux_control::declare_frontend_reporter!(
    pub(in crate::tui::app::event_loop);
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionView,
);
