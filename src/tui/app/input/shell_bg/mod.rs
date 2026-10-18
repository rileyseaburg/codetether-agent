//! Native presentation bindings for background chat shell commands.
mod view;
use crate::tui::app::state::App;
/// Drain a completed command into the visible transcript.
pub(crate) fn drain_shell_events(app: &mut App) -> bool {
    codetether_chat_shell::drain(app)
}
