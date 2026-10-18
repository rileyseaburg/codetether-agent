//! Goal-setup modal rendering.
use crate::{
    GoalPromptState, area::popup_area, footer::footer_line, goals::goal_lines,
    header::header_lines, input::input_line,
};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Render the goal-setup modal if one is active. No-op otherwise.
///
/// # Arguments
/// * `f` - Destination terminal frame.
/// * `area` - Area within which the modal is centered.
/// * `state` - Optional active prompt.
///
/// # Returns
/// Draws the prompt without modifying its input state.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, state: &codetether_goal_prompt::GoalPromptState) {
/// let area = f.area();
/// let pending = None;
/// codetether_goal_prompt::render_if_active(f, area, &pending);
/// # }
/// ```
pub fn render_if_active(f: &mut Frame, area: Rect, state: &Option<GoalPromptState>) {
    if let Some(state) = state {
        render(f, area, state);
    }
}

/// Render the goal-setup modal over the whole screen.
///
/// # Arguments
/// * `f` - Destination terminal frame.
/// * `area` - Area within which the modal is centered.
/// * `state` - Active prompt.
///
/// # Returns
/// Draws the prompt without modifying its input state.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, state: &codetether_goal_prompt::GoalPromptState) {
/// let area = f.area();
/// codetether_goal_prompt::render(f, area, state);
/// # }
/// ```
pub fn render(f: &mut Frame, area: Rect, state: &GoalPromptState) {
    let popup = popup_area(area);
    let mut text: Vec<Line<'static>> = header_lines(state);
    text.extend(goal_lines(state));
    text.push(Line::from(""));
    text.push(input_line(state));
    text.push(Line::from(""));
    text.push(footer_line());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Set up your goals ");
    let para = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(Clear, popup);
    f.render_widget(para, popup);
}
