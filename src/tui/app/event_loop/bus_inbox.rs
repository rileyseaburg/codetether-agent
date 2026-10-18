//! Frontend admission and dispatch callbacks for the inbox event-loop step.
codetether_a2a_interactive::declare_frontend_inbox_tick!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::provider::ProviderRegistry,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_runtime::TuiSessionHandle;
    crate::tui::app::inbox::live::admit,
    crate::tui::app::inbox::trigger_next,
);
