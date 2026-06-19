//! Centered, friendly modal card for the goal-setup prompt.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::tui::app::goal_prompt::GoalPromptState;

#[path = "goal_prompt_overlay_lines.rs"]
mod lines;
use lines::{footer_line, goal_lines, header_lines, input_line};

/// Render the goal-setup modal if one is active. No-op otherwise.
pub fn render_if_active(f: &mut Frame, area: Rect, state: &Option<GoalPromptState>) {
    if let Some(state) = state {
        render(f, area, state);
    }
}

/// Render the goal-setup modal over the whole screen.
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

fn popup_area(area: Rect) -> Rect {
    let width = area.width.min(72).max(area.width.min(40));
    let height = area.height.min(16).max(area.height.min(10));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
