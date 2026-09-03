//! Header and decision text for the approval popup.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::app::state::approval_queue::ApprovalSnapshot;

const DECISION_KEYS: &str =
    "Ctrl+E edit · feedback+Enter denies · Ctrl+Y copy · Ctrl+A approve · Ctrl+D deny";

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
            Line::from(DECISION_KEYS),
            Line::from(format!("id: {} | queued: {count}", item.id)),
        ]),
        area,
    );
}

#[cfg(test)]
#[test]
fn feedback_guidance_names_denial_effect() {
    assert!(DECISION_KEYS.contains("feedback+Enter denies"));
}