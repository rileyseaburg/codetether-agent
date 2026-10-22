//! Render recent runtime events below the live reply in both chat layouts.
use crate::tui::app::state::AppState;
use ratatui::{style::Stylize, text::Line};

pub(super) fn append(lines: &mut Vec<Line<'static>>, state: &AppState, width: usize) {
    if state.activity_events.entries.is_empty() {
        return;
    }
    lines.push(Line::default());
    lines.push(Line::from("Recent events".cyan()));
    for entry in &state.activity_events.entries {
        let at = codetether_chat_status::format_timestamp(entry.at);
        let count = if entry.count > 1 {
            format!(" ×{}", entry.count)
        } else {
            String::new()
        };
        let label = format!("{at} {}{count}", entry.label);
        // Wrap labels instead of dropping their distinguishing suffixes.
        let chars: Vec<char> = label.chars().collect();
        for chunk in chars.chunks(width.max(1)) {
            lines.push(Line::from(chunk.iter().collect::<String>().dim()));
        }
    }
}
