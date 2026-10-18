//! Windowing and chrome around already-rendered activity rows.
use super::{
    ToolPanelRender,
    panel_chrome::{panel_footer, panel_header},
    panel_window::{pad_blank_rows, panel_window},
};
use codetether_chat_model::ChatMessage;
use codetether_chat_status::format_timestamp;
use ratatui::text::Line;
pub(super) fn finish(
    messages: &[&ChatMessage],
    body_lines: Vec<Line<'static>>,
    scroll_offset: usize,
    header_width: usize,
) -> ToolPanelRender {
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
