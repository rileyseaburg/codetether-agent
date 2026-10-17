//! Question modal with scrolling choices and a custom-answer row.
use crate::tui::app::state::App;
use codetether_question::runtime;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    style::Stylize,
    widgets::{Block, Clear, Paragraph, Wrap},
};
#[path = "question_choices.rs"]
mod choices;

pub(super) fn render(frame: &mut Frame, app: &App) {
    let Some(id) = app.state.session_id.as_deref() else {
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
    choices::help(frame, rows[2], q.multiple);
}

#[cfg(test)]
#[path = "question_tests.rs"]
mod tests;
