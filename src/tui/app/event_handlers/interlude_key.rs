//! Native play-break state binding for key input.
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
pub(super) fn handle(app: &mut App, key: KeyEvent) -> bool {
    codetether_interlude::interactive::handle(
        &mut app.state.interlude,
        &mut app.state.needs_redraw,
        key,
    )
}
