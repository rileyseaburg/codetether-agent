//! Top-level builder for the scrollable tool activity panel.

use std::time::Instant;

use ratatui::text::Line;

use super::super::status_bar::format_timestamp;
use super::ToolPanelRender;
use super::item::render_tool_activity_item;
use super::panel_chrome::{empty_body_lines, panel_footer, panel_header};
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

    let visible_lines = super::TOOL_PANEL_VISIBLE_LINES.min(body_lines.len()).max(1);
    let max_scroll = body_lines.len().saturating_sub(visible_lines);
    // Sentinel ≥ 1_000_000 means "auto-follow latest"; else clamp to max_scroll.
    let start = if scroll_offset >= 1_000_000 {
        max_scroll
    } else {
        scroll_offset.min(max_scroll)
    };
    let end = (start + visible_lines).min(body_lines.len());
    let timestamp = messages
        .first()
        .map(|message| format_timestamp(message.timestamp))
        .unwrap_or_else(|| "--:--:--".to_string());
    let total = body_lines.len();
    let mut lines: Vec<Line<'static>> = vec![panel_header(
        &timestamp,
        messages.len(),
        start,
        end,
        total,
        max_scroll,
        header_width,
    )];
    lines.extend(body_lines[start..end].iter().cloned());
    lines.push(panel_footer(start, max_scroll, header_width));
    ToolPanelRender { lines, max_scroll }
}
