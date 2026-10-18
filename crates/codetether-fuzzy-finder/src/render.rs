//! Fuzzy finder render presentation.
use crate::FuzzyFindState;
use crate::{area::popup_area, row::result_line, window::window_offset};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
};

/// Render the fuzzy finder modal over the whole screen.
/// # Arguments
/// * `f` - Target frame.
/// * `area` - Available screen rectangle.
/// * `state` - Current query and ranked selection.
/// # Returns
/// Unit after drawing the centered modal.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let state = codetether_fuzzy_finder::FuzzyFindState::new(vec![]);
/// codetether_fuzzy_finder::render(f, area, &state);
/// # }
/// ```
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
