//! Frontend selection for cursor-gated worker synchronization.
codetether_tui_tick::declare_frontend_worker_sync!(
    pub(in crate::tui::app::event_loop);
    crate::tui::app::state::App,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::worker_bridge::sync_worker_bridge_agents,
);
