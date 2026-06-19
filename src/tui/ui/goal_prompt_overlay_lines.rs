//! Line builders for the goal-setup modal card.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::tui::app::goal_prompt::GoalPromptState;

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

pub(super) fn input_line(state: &GoalPromptState) -> Line<'static> {
    Line::from(vec![
        Span::styled("> ", Style::default().fg(Color::Cyan)),
        Span::raw(state.current.clone()),
        Span::styled("\u{2588}", Style::default().fg(Color::Cyan)),
    ])
}

pub(super) fn footer_line() -> Line<'static> {
    Line::from(Span::styled(
        "Enter = add  •  empty Enter = done  •  Esc = skip",
        Style::default().fg(Color::DarkGray),
    ))
}
