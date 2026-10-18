//! Persistent, width-aware decision and navigation hints.

use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Paragraph, Wrap},
};

const DECISIONS: &str = "Ctrl+A approve · Ctrl+D deny";
const NAVIGATION: &str = "↑↓ PgUp/PgDn Home/End scroll · Tab/Shift+Tab requests";
const EDITS: &str = "Ctrl+E edit · Ctrl+Y copy · feedback+Enter denies";

fn paragraph() -> Paragraph<'static> {
    Paragraph::new(vec![
        Line::from(DECISIONS),
        Line::from(NAVIGATION),
        Line::from(EDITS),
    ])
    .wrap(Wrap { trim: false })
}

/// Reserve the rows occupied by wrapped shortcuts at the current width.
pub(super) fn height(width: u16) -> u16 {
    u16::try_from(paragraph().line_count(width)).unwrap_or(u16::MAX)
}

/// Keep decision keys outside the scrollable request details.
pub(super) fn render(f: &mut Frame, area: Rect) {
    f.render_widget(paragraph(), area);
}

#[cfg(test)]
#[test]
fn approval_feedback_guidance_names_denial_effect() {
    assert!(EDITS.contains("feedback+Enter denies"));
    assert!(height(36) > height(100));
}
