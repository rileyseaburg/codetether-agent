//! Application visibility adapter for the play-break overlay.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
/// Draw the active play break while a request is processing.
pub fn render_if_active(f: &mut Frame, area: Rect, app: &mut App) {
    if !app.state.processing {
        return;
    }
    let Some(game) = app.state.interlude.as_mut() else {
        return;
    };
    codetether_interlude::render(f, area, game);
}
