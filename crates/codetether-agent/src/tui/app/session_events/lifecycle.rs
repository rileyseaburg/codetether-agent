//! Frontend type and callbacks for shared session-event lifecycles.
use super::host::Native;
codetether_session_events::declare_event_lifecycle!(
    Native;
    crate::session::Session,
    crate::session::SessionEvent,
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::worker_bridge::TuiWorkerBridge;
    crate::tui::app::worker_bridge::handle_processing_started,
    crate::tui::app::worker_bridge::handle_processing_stopped,
    crate::tui::app::inbox::live::settle_from_session,
);
