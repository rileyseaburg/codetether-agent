//! Delegate native chat events to shared presentation policy.
use crate::{session::SessionEvent, tui::app::state::App};
pub use codetether_chat_events::attach_usage_to_last_completion_message;
pub(super) fn handle_event(app: &mut App, evt: SessionEvent) -> Option<SessionEvent> {
    codetether_chat_events::usage::handle_event(&mut super::chat_host::Native(app), evt)
}
