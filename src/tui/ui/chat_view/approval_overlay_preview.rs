//! Scrollable invocation preview inside the approval popup.

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub(super) fn render(
    f: &mut Frame,
    area: Rect,
    preview: Option<&str>,
    resource: &str,
    scroll: u16,
) {
    let content = preview.unwrap_or(resource);
    let inner = Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    );
    let max_scroll = super::approval_preview_scroll::max_offset(content, inner.width, inner.height);
    let scroll = scroll.min(max_scroll);
    let lines = super::approval_diff::lines(content);
    let title = format!("Preview · ↑↓/PgUp/PgDn · row {}", scroll + 1);
    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(paragraph, area);
}
