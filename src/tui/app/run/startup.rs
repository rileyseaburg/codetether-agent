//! Native type specialization for shared startup results.
pub(super) type Startup = codetether_tui_startup::Startup<
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::session::TailLoad,
>;
