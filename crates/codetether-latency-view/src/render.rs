//! Inspector layout and ordered section assembly.
use crate::{LatencyView, format::section_heading};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};
/// Draw the latency inspector using projected state and deferred token lines.
/// # Arguments
/// * `view` - Current request, tool and chat-turn data.
/// * `token_lines` - Existing application token presentation, called once.
/// * `f` - Target frame.
/// * `area` - Available display area.
/// # Returns
/// Unit after rendering the inspector and footer.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect, view: &codetether_latency_view::LatencyView<'_>) {
/// codetether_latency_view::render_latency(f, area, view, Vec::new);
/// # }
/// ```
pub fn render_latency(
    f: &mut Frame,
    area: Rect,
    view: &LatencyView<'_>,
    token_lines: impl FnOnce() -> Vec<String>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);
    let mut lines = crate::header::lines(view);
    crate::current::append(&mut lines, view);
    crate::last_request::append(&mut lines, view);
    crate::completion::append(&mut lines, view);
    crate::last_tool::append(&mut lines, view);
    lines.push(Line::from(""));
    lines.push(section_heading("Token Usage Snapshot"));
    for line in token_lines().into_iter().take(10) {
        lines.push(Line::from(line));
    }
    crate::providers::append(&mut lines);
    crate::tools::append(&mut lines, view);
    crate::chat::push_chat_turn_latency(&mut lines, view.chat_latency);
    crate::recent::append(&mut lines, view);
    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Latency"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    crate::footer::render(f, chunks[1], view);
}
