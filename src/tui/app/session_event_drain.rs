//! Application bindings for bounded queued session-event delivery.

codetether_session_events::declare_event_drain!(
    crate::session::Session,
    crate::session::SessionEvent,
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::tui::app::session_events::handle_session_event,
);

#[cfg(test)]
use crate::session::SessionEvent;
#[cfg(test)]
use crate::tui::app::state::App;
#[cfg(test)]
use tokio::sync::mpsc;
#[cfg(test)]
mod tests;
