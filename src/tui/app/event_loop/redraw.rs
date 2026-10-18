//! Native rendering binding for shared streaming redraw policy.
mod host;
use crate::tui::app::{session_runtime::SessionView, state::App};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::Instant;
pub(super) fn draw_if_ready(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &SessionView,
    last_draw: &mut Option<Instant>,
) -> anyhow::Result<()> {
    codetether_tui_tick::redraw::draw_if_ready(
        &mut host::Native {
            terminal,
            app,
            session,
        },
        last_draw,
    )
}
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
