//! Scrollable request details inside the approval popup.

use crate::tui::app::state::approval_queue::{self, ApprovalSnapshot};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub(super) fn render(f: &mut Frame, area: Rect, item: &ApprovalSnapshot, scroll: &mut u16) {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);
    let paragraph =
        Paragraph::new(super::approval_overlay_content::lines(item)).wrap(Wrap { trim: false });
    let rows = paragraph.line_count(inner.width);
    let limit = super::approval_preview_scroll::offset_for_rows(rows, inner.height);
    approval_queue::set_scroll_limit(&item.id, limit);
    *scroll = (*scroll).min(limit);
    let start = usize::from(*scroll) + 1;
    let end = (usize::from(*scroll) + usize::from(inner.height)).min(rows);
    let title = format!("Details · rows {start}–{end}/{rows}");
    f.render_widget(
        paragraph.block(block.title(title)).scroll((*scroll, 0)),
        area,
    );
}
