//! TUI adapter for session runtime events.
//!
//! This module is the boundary between the session runtime and the terminal UI.
//! It receives [`crate::session::SessionEvent`] values produced by the runtime, routes them
//! through the event-processing pipeline, updates UI retention state, and then
//! applies the resulting event to the visible application context.
//!
//! The adapter is intentionally thin: event-specific behavior lives in the
//! sibling modules below, while this module coordinates ordering and shared UI
//! side effects such as marking activity and keeping the chat view pinned to
//! the newest message when auto-follow is enabled.

mod context;
mod errors;
mod interlude;
mod lifecycle;
mod retention;
mod tools;

#[cfg(test)]
mod tests;

codetether_chat_events::declare_chat_state!(
    crate::tui::app::state::App,
    crate::session::SessionEvent,
    crate::tui::constants::MAX_STREAMING_TEXT_BYTES,
);
pub use usage::attach_usage_to_last_completion_message;

codetether_session_events::declare_event_pipeline!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionSlot,
    crate::tui::worker_bridge::TuiWorkerBridge,
    crate::session::SessionEvent;
    crate::tui::app::session_events::retention::trim,
    crate::tui::app::session_events::context::handle_event;
    crate::tui::app::session_events::lifecycle::handle_event,
    crate::tui::app::session_events::interlude::handle_event,
    crate::tui::app::session_events::retry::handle_event,
    crate::tui::app::session_events::tools::handle_event,
    crate::tui::app::session_events::text_dispatch::handle_event,
    crate::tui::app::session_events::usage::handle_event,
    crate::tui::app::session_events::errors::handle_event,
);

#[cfg(test)]
use crate::tui::app::state::App;
