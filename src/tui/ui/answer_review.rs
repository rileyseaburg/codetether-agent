//! Application visibility guard for the satisfaction popup.
#[path = "answer_review_content.rs"]
mod content;
use crate::session::tasks::runtime::answer_review;
use crate::tui::app::state::App;
use ratatui::Frame;
pub(crate) fn render(frame: &mut Frame, app: &App) {
    let Some(session) = app.state.session_id.as_deref() else {
        return;
    };
    if !answer_review::ready(session)
        || app.state.input.trim_start().starts_with('/')
        || app.state.goal_editor.is_some()
    {
        return;
    }

    codetether_answer_review_view::render(frame, app.state.answer_review_yes);
}
#[cfg(test)]
#[path = "answer_review_tests.rs"]
mod tests;
