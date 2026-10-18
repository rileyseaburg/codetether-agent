//! Frontend type and callbacks for shared error-event handling.
use super::host::Native;
codetether_chat_events::declare_error_events!(
    Native;
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::session::SessionEvent;
    crate::tui::app::inbox::live::settle,
    codetether_model_failover::schedule::schedule,
);
