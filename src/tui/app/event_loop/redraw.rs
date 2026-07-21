//! Streaming redraw rate limiting.

use std::time::{Duration, Instant};

use ratatui::{Terminal, backend::CrosstermBackend};

use crate::tui::app::{session_runtime::SessionView, state::App};

const STREAM_FRAME_INTERVAL: Duration = Duration::from_millis(33);

pub(super) fn draw_if_ready(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    session: &SessionView,
    last_draw: &mut Option<Instant>,
) -> anyhow::Result<()> {
    if !app.state.needs_redraw || !ready(app, *last_draw) {
        return Ok(());
    }
    crate::tui::app::safe_draw::draw_ui(terminal, app, session)?;
    app.state.needs_redraw = false;
    *last_draw = Some(Instant::now());
    Ok(())
}

pub(super) fn ready(app: &App, last_draw: Option<Instant>) -> bool {
    let Some(last_draw) = last_draw else {
        return true;
    };
    if !app.state.processing || input_since(app, last_draw) {
        return true;
    }
    last_draw.elapsed() >= STREAM_FRAME_INTERVAL
}

fn input_since(app: &App, last_draw: Instant) -> bool {
    app.state
        .last_key_at
        .is_some_and(|last_input| last_input >= last_draw)
}

#[cfg(test)]
#[path = "redraw_tests.rs"]
mod tests;
