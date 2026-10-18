//! Question title, prompt, and panel layout.
use crate::choices;
use codetether_question::runtime;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    style::Stylize,
    widgets::{Block, Clear, Paragraph, Wrap},
};
/// Draw the active question for a session, if one is pending.
///
/// # Arguments
/// * `frame` - Destination frame.
/// * `session_id` - Current session, or none when not attached.
///
/// # Returns
/// Draws the pending modal without changing the draft.
///
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame) {
/// codetether_question_view::render_pending(frame, Some("session"));
/// # }
/// ```
pub fn render_pending(frame: &mut Frame, session_id: Option<&str>) {
    let Some(id) = session_id else {
        return;
    };
    let Some(draft) = runtime::pending(id) else {
        return;
    };
    let q = draft.question();
    let area = frame.area().inner(Margin::new(2, 1));
    let title = format!(
        " Question {}/{} · {} ",
        draft.index() + 1,
        draft.total(),
        q.header
    );
    let block = Block::bordered().title(title.cyan().bold());
    let inner = block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    let rows = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
    .split(inner);
    frame.render_widget(
        Paragraph::new(q.question.as_str()).wrap(Wrap { trim: false }),
        rows[0],
    );
    choices::render(frame, rows[1], &draft);
    crate::help::render(frame, rows[2], q.multiple);
}
