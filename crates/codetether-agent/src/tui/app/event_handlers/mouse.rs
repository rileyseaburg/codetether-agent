//! Native binding for shared mouse-event navigation policy.
use crate::tui::app::{navigation::Navigation, state::App};
use crossterm::event::MouseEvent;
use std::path::Path;
mod host;
/// Dispatch one mouse event through shared routing and existing native effects.
pub async fn handle_mouse_event(app: &mut App, cwd: &Path, mouse: MouseEvent) {
    codetether_tui_navigation::mouse::handle_mouse_event(&mut Navigation(app), cwd, mouse).await;
}
