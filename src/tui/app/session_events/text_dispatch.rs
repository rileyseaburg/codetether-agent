//! Delegate native chat events to shared presentation policy.
use crate::{session::SessionEvent, tui::app::state::App};
pub(super) fn handle_event(app: &mut App, evt: SessionEvent) -> Option<SessionEvent> {
    codetether_chat_events::handle_text_event(&mut super::chat_host::Native(app), evt)
}
