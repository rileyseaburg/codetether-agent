//! Centered modal card for the `/edit` fuzzy file finder.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::tui::app::fuzzy_find::FuzzyFindState;

/// Render the fuzzy finder if active. No-op otherwise.
pub fn render_if_active(f: &mut Frame, area: Rect, state: &Option<FuzzyFindState>) {
    if let Some(state) = state {
        render(f, area, state);
    }
}

/// Render the fuzzy finder modal over the whole screen.
pub fn render(f: &mut Frame, area: Rect, state: &FuzzyFindState) {
    let popup = popup_area(area);
    let rows = popup.height.saturating_sub(3) as usize;
    let offset = window_offset(state.selected, rows, state.results.len());
    let mut text: Vec<Line<'static>> = vec![Line::from(format!("> {}", state.query))];
    for (i, entry) in state.results.iter().enumerate().skip(offset).take(rows) {
        text.push(result_line(i == state.selected, &entry.display));
    }
    let title = format!(" Open file ({} matches) ", state.results.len());
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title);
    f.render_widget(Clear, popup);
    f.render_widget(Paragraph::new(text).block(block), popup);
}

fn result_line(selected: bool, display: &str) -> Line<'static> {
    let prefix = if selected { "▶ " } else { "  " };
    let style = if selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    Line::styled(format!("{prefix}{display}"), style)
}

/// Scroll offset that keeps `selected` visible within a window of `rows`.
fn window_offset(selected: usize, rows: usize, total: usize) -> usize {
    if rows == 0 || total <= rows {
        return 0;
    }
    let max_offset = total - rows;
    selected.saturating_sub(rows - 1).min(max_offset)
}

fn popup_area(area: Rect) -> Rect {
    let width = area.width.min(90).max(area.width.min(50));
    let height = area.height.min(20).max(area.height.min(8));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
