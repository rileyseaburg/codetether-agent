//! Pending approval popup for the chat view.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::tui::app::state::approval_queue;

pub(crate) fn render(f: &mut Frame, area: Rect) {
    let Some(item) = approval_queue::active() else {
        return;
    };
    let count = approval_queue::len();
    let heading = format!("{} wants to {} {}", item.tool, item.action, item.resource);
    let reason = item.reason;
    let id = item.id;
    let popup = popup_area(area);
    let text = vec![
        Line::from(Span::styled(
            heading,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(reason),
        Line::from(""),
        Line::from("Ctrl+A approve | Ctrl+D deny | /approve session | /abort"),
        Line::from(format!("id: {id} | queued: {count}")),
    ];
    let block = Block::default().borders(Borders::ALL).title("Approval");
    let para = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(Clear, popup);
    f.render_widget(para, popup);
}

fn popup_area(area: Rect) -> Rect {
    let width = area.width.min(76).max(area.width.min(36));
    let height = 7u16;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + area.height.saturating_sub(height + 2);
    Rect::new(x, y, width, height)
}
