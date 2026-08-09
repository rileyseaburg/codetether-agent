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
    let max_scroll = content.lines().count().saturating_sub(1) as u16;
    let lines = super::approval_diff::lines(content);
    let title = format!(
        "Preview · ↑↓/PgUp/PgDn · line {}",
        scroll.min(max_scroll) + 1
    );
    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false })
        .scroll((scroll.min(max_scroll), 0));
    f.render_widget(paragraph, area);
}
