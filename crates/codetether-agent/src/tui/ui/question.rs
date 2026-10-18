//! Application adapter for the shared question modal.
use crate::tui::app::state::App;
#[cfg(test)]
use codetether_question::runtime;
pub(super) fn render(frame: &mut ratatui::Frame, app: &App) {
    codetether_question_view::render_pending(frame, app.state.session_id.as_deref());
}
#[cfg(test)]
#[path = "question_tests.rs"]
mod tests;
