//! input lines for the goal-setup card.
use crate::GoalPromptState;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(super) fn input_line(state: &GoalPromptState) -> Line<'static> {
    Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Cyan)),
        Span::raw(state.current.clone()),
        Span::styled("\u{2588}", Style::default().fg(Color::Cyan)),
    ])
}
