//! Frontend resource selection for ordered loop shutdown.
codetether_tui_reactor::declare_frontend_loop_shutdown!(
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle,
    crate::tui::app::event_loop::setup::mux_status::Reporter,
    codetether_tui_worker::shutdown_bridge,
);
