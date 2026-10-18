//! Native projection into the shared recent-session panel.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    codetether_session_picker::recent::render(f, &app.state.session_picker, area);
}
