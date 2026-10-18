//! Native bindings for shared context telemetry presentation.
use crate::{session::SessionEvent, tui::app::state::App};
#[path = "context_host.rs"]
mod host;
pub(super) fn handle_event(app: &mut App, evt: SessionEvent) {
    codetether_context_health::events::handle_event(&mut host::Native(app), evt);
}
