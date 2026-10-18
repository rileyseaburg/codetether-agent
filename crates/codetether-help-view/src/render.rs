//! Help render content and presentation.
use crate::HelpScrollState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
/// Draw help rows, clamp scrolling and render a scrollbar when needed.
/// # Arguments
/// * `f` - Target frame.
/// * `area` - Popup bounds.
/// * `lines` - Owned help rows.
/// * `scroll` - Scroll offset, clamped to current content height.
/// # Returns
/// Unit after rendering the popup and optional scrollbar.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let mut scroll = codetether_help_view::HelpScrollState::default();
/// codetether_help_view::render_help_overlay(f, area, vec![], &mut scroll);
/// # }
/// ```
pub fn render_help_overlay(
    f: &mut Frame,
    area: Rect,
    lines: Vec<Line<'static>>,
    scroll: &mut HelpScrollState,
) {
    let total_lines = lines.len();
    let content_height = area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(content_height);
    if scroll.offset > max_scroll {
        scroll.offset = max_scroll;
    }
    let scroll_offset = scroll.offset;

    f.render_widget(Clear, area);
    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset as u16, 0));
    f.render_widget(widget, area);

    if total_lines > content_height {
        let mut sb_state = ScrollbarState::new(total_lines).position(scroll_offset);
        f.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            area,
            &mut sb_state,
        );
    }
}
