//! Frontend selection for shared redraw policy.
codetether_tui_tick::declare_frontend_redraw!(
    crate::tui::app::state::App,
    crate::tui::app::session_runtime::SessionView,
    ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    crate::tui::app::safe_draw::draw_ui,
);
#[cfg(test)]
use crate::tui::app::state::App;
#[cfg(test)]
use std::time::Instant;
#[cfg(test)]
fn finish(app: &mut App, last_draw: &mut Option<Instant>, rendered: bool) {
    codetether_tui_tick::redraw::finish(&mut app.state.needs_redraw, last_draw, rendered);
}
#[cfg(test)]
pub(super) fn ready(app: &App, last_draw: Option<Instant>) -> bool {
    codetether_tui_tick::redraw::ready(app.state.processing, app.state.last_key_at, last_draw)
}
#[cfg(test)]
#[path = "redraw_tests.rs"]
mod tests;
