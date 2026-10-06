//! Render a satisfaction choice without granting tool approval authority.

#[path = "answer_review_content.rs"]
mod content;

use crate::session::tasks::runtime::answer_review;
use crate::tui::app::state::App;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
};

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
    let area = frame.area();
    let width = area.width.min(74);
    let height = area.height.min(6);
    let popup = Rect::new(
        area.x + (area.width - width) / 2,
        area.y + area.height - height,
        width,
        height,
    );
    let lines = content::lines(app.state.answer_review_yes);
    frame.render_widget(Clear, popup);
    frame.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Answer satisfaction"),
        ),
        popup,
    );
}

#[cfg(test)]
#[path = "answer_review_tests.rs"]
mod tests;
