//! header lines for the goal-setup card.
use crate::GoalPromptState;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub(super) fn header_lines(state: &GoalPromptState) -> Vec<Line<'static>> {
    vec![
        Line::from(Span::styled(
            state.title.clone(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            state.body.clone(),
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ]
}
