//! goals lines for the goal-setup card.
use crate::GoalPromptState;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(super) fn goal_lines(state: &GoalPromptState) -> Vec<Line<'static>> {
    if state.goals.is_empty() {
        return vec![Line::from(Span::styled(
            "No goals yet — type one below and press Enter.",
            Style::default().fg(Color::DarkGray),
        ))];
    }
    state
        .goals
        .iter()
        .enumerate()
        .map(|(i, g)| {
            Line::from(vec![
                Span::styled(format!("  {}. ", i + 1), Style::default().fg(Color::Green)),
                Span::raw(g.clone()),
            ])
        })
        .collect()
}
