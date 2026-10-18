//! Native bindings for top-level TUI rendering.
mod bus;
mod host;
use crate::tui::app::{session_runtime::SessionView, state::App};
use ratatui::Frame;
/// Top-level entry point called by the TUI event loop on every frame.
///
/// Dispatches to the active [`crate::tui::models::ViewMode`] renderer and renders overlays.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::main::ui;
/// # fn demo(f: &mut ratatui::Frame, app: &mut codetether_agent::tui::app::state::App) {
/// let view = codetether_agent::tui::app::session_runtime::SessionView::default();
/// ui(f, app, &view);
/// # }
/// ```
pub fn ui(f: &mut Frame, app: &mut App, session: &SessionView) {
    codetether_tui_render::render(f, &mut host::Native { app, session });
}
