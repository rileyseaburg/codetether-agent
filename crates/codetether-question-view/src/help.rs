//! Keyboard help for single and multiple selection.
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    widgets::{Paragraph, Wrap},
};

pub(super) fn render(frame: &mut Frame, area: Rect, multiple: bool) {
    let help = if multiple {
        "↑↓/Tab move · Space toggle · Enter next/submit\nType custom answer · Shift+Tab back · Esc dismiss · Ctrl+C cancel turn"
    } else {
        "↑↓/Tab move · Enter choose/next\nType custom answer · Shift+Tab back · Esc dismiss · Ctrl+C cancel turn"
    };
    frame.render_widget(Paragraph::new(help).dim().wrap(Wrap { trim: true }), area);
}
