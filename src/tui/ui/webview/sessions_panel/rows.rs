//! Title and metadata rows for webview session entries.

use ratatui::prelude::*;

use crate::session::SessionSummary;

pub(super) fn title(idx: usize, selected: usize, summary: &SessionSummary) -> Line<'static> {
    let marker = if idx == selected { "●" } else { "○" };
    let style = if idx == selected {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default().fg(Color::Gray)
    };
    let label = summary.display_label();
    let label = crate::util::truncate_bytes_safe(&label, 22);
    Line::from(Span::styled(format!("{marker} {label}"), style))
}

pub(super) fn metadata(summary: &SessionSummary) -> String {
    format!(
        "{} msgs • {}",
        summary.message_count,
        summary.updated_at.format("%m-%d %H:%M")
    )
}
