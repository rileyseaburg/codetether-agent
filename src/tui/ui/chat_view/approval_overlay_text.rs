//! Header and decision text for the approval popup.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::app::state::approval_queue::ApprovalSnapshot;

pub(super) fn header(f: &mut Frame, area: Rect, item: &ApprovalSnapshot) {
    f.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(
                format!("{} wants to {}", item.tool, item.action),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("→ {}", item.reason)),
        ]),
        area,
    );
}

pub(super) fn footer(f: &mut Frame, area: Rect, item: &ApprovalSnapshot, count: usize) {
    f.render_widget(
        Paragraph::new(vec![
            Line::from(
                "Ctrl+E edit code · type feedback · Ctrl+Y copy · Ctrl+A approve · Ctrl+D deny",
            ),
            Line::from(format!("id: {} | queued: {count}", item.id)),
        ]),
        area,
    );
}
