//! Top-level builder for the scrollable tool activity panel.

use std::time::Instant;

use ratatui::text::Line;

use super::super::status_bar::format_timestamp;
use super::ToolPanelRender;
use super::item::render_tool_activity_item;
use super::panel_chrome::{empty_body_lines, panel_footer, panel_header};
use super::panel_window::{pad_blank_rows, panel_window};
use super::pending_spinner::append_pending_tool;
use crate::tui::chat::message::ChatMessage;

/// Snapshot of the in-flight tool call (None when no tool is running).
pub type PendingToolSnapshot<'a> = Option<(&'a str, Instant)>;

pub fn build_tool_activity_panel(
    messages: &[&ChatMessage],
    scroll_offset: usize,
    width: usize,
    pending: PendingToolSnapshot<'_>,
) -> ToolPanelRender {
    let header_width = width.max(24);
    let preview_width = header_width.saturating_sub(10).max(24);
    let mut body_lines = Vec::new();

    for message in messages {
        render_tool_activity_item(&mut body_lines, message, preview_width);
    }
    if let Some((name, started_at)) = pending {
        append_pending_tool(&mut body_lines, name, started_at, None);
    }
    if body_lines.is_empty() {
        body_lines = empty_body_lines();
    }

    let window = panel_window(body_lines.len(), scroll_offset);
    let timestamp = messages
        .first()
        .map(|message| format_timestamp(message.timestamp))
        .unwrap_or_else(|| "--:--:--".to_string());
    let mut lines: Vec<Line<'static>> = vec![panel_header(
        &timestamp,
        messages.len(),
        window.start,
        window.end,
        window.total,
        window.max_scroll,
        header_width,
    )];
    lines.extend(body_lines[window.start..window.end].iter().cloned());
    let shown = window.end.saturating_sub(window.start);
    pad_blank_rows(&mut lines, window.visible_lines.saturating_sub(shown));
    lines.push(panel_footer(window.start, window.max_scroll, header_width));
    ToolPanelRender {
        lines,
        max_scroll: window.max_scroll,
    }
}
