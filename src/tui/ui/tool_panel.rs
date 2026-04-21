use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::status_bar::format_timestamp;
use crate::tui::app::text::truncate_preview;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;

/// Max visible lines inside the compact tool panel.
pub const TOOL_PANEL_VISIBLE_LINES: usize = 6;
/// Max preview lines captured for a single tool activity item.
const TOOL_PANEL_ITEM_MAX_LINES: usize = 18;
/// Max bytes processed for a single tool activity preview.
const TOOL_PANEL_ITEM_MAX_BYTES: usize = 6_000;

pub struct RenderEntry<'a> {
    pub tool_activity: Vec<&'a ChatMessage>,
    pub message: Option<&'a ChatMessage>,
}

pub struct ToolPanelRender {
    pub lines: Vec<Line<'static>>,
    pub max_scroll: usize,
}

pub fn build_render_entries(messages: &[ChatMessage]) -> Vec<RenderEntry<'_>> {
    let mut entries = Vec::new();
    let mut pending_tool_activity = Vec::new();

    for message in messages {
        if is_tool_activity(&message.message_type) {
            pending_tool_activity.push(message);
            continue;
        }

        entries.push(RenderEntry {
            tool_activity: std::mem::take(&mut pending_tool_activity),
            message: Some(message),
        });
    }

    if !pending_tool_activity.is_empty() {
        entries.push(RenderEntry {
            tool_activity: pending_tool_activity,
            message: None,
        });
    }

    entries
}

pub fn is_tool_activity(message_type: &MessageType) -> bool {
    matches!(
        message_type,
        MessageType::ToolCall { .. } | MessageType::ToolResult { .. } | MessageType::Thinking(_)
    )
}

pub fn separator_pattern(entry: &RenderEntry<'_>) -> &'static str {
    match entry.message.map(|message| &message.message_type) {
        Some(MessageType::System | MessageType::Error) | None => "·",
        _ => "─",
    }
}

pub fn render_chat_message(
    lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) {
    match &message.message_type {
        MessageType::User => render_formatted_message(
            lines,
            message,
            formatter,
            "user",
            "▸ ",
            palette.get_message_color("user"),
        ),
        MessageType::Assistant => render_formatted_message(
            lines,
            message,
            formatter,
            "assistant",
            "◆ ",
            palette.get_message_color("assistant"),
        ),
        MessageType::System => render_formatted_message(
            lines,
            message,
            formatter,
            "system",
            "⚙ ",
            palette.get_message_color("system"),
        ),
        MessageType::Error => render_formatted_message(
            lines,
            message,
            formatter,
            "error",
            "✖ ",
            palette.get_message_color("error"),
        ),
        MessageType::Image { .. } => {
            let timestamp = format_timestamp(message.timestamp);
            lines.push(Line::from(vec![
                Span::styled(
                    format!("[{timestamp}] "),
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(ratatui::style::Modifier::DIM),
                ),
                Span::styled("🖼️  image", Style::default().fg(Color::Cyan).italic()),
            ]));
            lines.push(Line::from(Span::styled(
                format!("  {}", truncate_preview(&message.content, 120)),
                Style::default().fg(Color::Cyan).dim(),
            )));
        }
        MessageType::File { path, size } => {
            let timestamp = format_timestamp(message.timestamp);
            let size_label = size.map(|s| format!(" ({s} bytes)")).unwrap_or_default();
            lines.push(Line::from(vec![
                Span::styled(
                    format!("[{timestamp}] "),
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(ratatui::style::Modifier::DIM),
                ),
                Span::styled(
                    format!("📎 file: {path}{size_label}"),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
        }
        MessageType::ToolCall { .. }
        | MessageType::ToolResult { .. }
        | MessageType::Thinking(_) => {}
    }
}

fn render_formatted_message(
    lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    formatter: &MessageFormatter,
    label: &str,
    icon: &str,
    color: Color,
) {
    let timestamp = format_timestamp(message.timestamp);
    let mut header_spans = vec![
        Span::styled(
            format!("[{timestamp}] "),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(ratatui::style::Modifier::DIM),
        ),
        Span::styled(icon.to_string(), Style::default().fg(color).bold()),
        Span::styled(label.to_string(), Style::default().fg(color).bold()),
    ];
    if let Some(u) = message.usage.as_ref() {
        header_spans.push(Span::styled(
            format!(
                "  · {} in / {} out · {}",
                format_tokens(u.prompt_tokens),
                format_tokens(u.completion_tokens),
                format_latency(u.duration_ms),
            ),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(ratatui::style::Modifier::DIM),
        ));
    }
    lines.push(Line::from(header_spans));
    let formatted = crate::tui::ui::chat_view::format_cache::format_message_cached(
        message,
        label,
        formatter,
        formatter.max_width(),
    );
    for line in formatted {
        let mut spans = vec![Span::styled("  ", Style::default().fg(color))];
        spans.extend(line.spans.into_iter());
        lines.push(Line::from(spans));
    }
}

fn format_tokens(n: usize) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}k", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_latency(ms: u64) -> String {
    if ms >= 1_000 {
        format!("{:.1}s", ms as f64 / 1_000.0)
    } else {
        format!("{ms}ms")
    }
}

pub fn build_tool_activity_panel(
    messages: &[&ChatMessage],
    scroll_offset: usize,
    width: usize,
) -> ToolPanelRender {
    let header_width = width.max(24);
    let preview_width = header_width.saturating_sub(10).max(24);
    let mut body_lines = Vec::new();

    for message in messages {
        render_tool_activity_item(&mut body_lines, message, preview_width);
    }

    if body_lines.is_empty() {
        body_lines.push(Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
            Span::styled(
                "No tool activity captured",
                Style::default().fg(Color::DarkGray).dim(),
            ),
        ]));
    }

    let visible_lines = TOOL_PANEL_VISIBLE_LINES.min(body_lines.len()).max(1);
    let max_scroll = body_lines.len().saturating_sub(visible_lines);
    let start = scroll_offset.min(max_scroll);
    let end = (start + visible_lines).min(body_lines.len());
    let mut lines = Vec::new();
    let timestamp = messages
        .first()
        .map(|message| format_timestamp(message.timestamp))
        .unwrap_or_else(|| "--:--:--".to_string());
    let scroll_label = if max_scroll == 0 {
        format!("{} lines", body_lines.len())
    } else {
        format!("{}-{} / {}", start + 1, end, body_lines.len())
    };
    let header = format!("[{timestamp}] ▣ tools {} • {scroll_label}", messages.len());
    lines.push(Line::from(Span::styled(
        truncate_preview(&header, header_width),
        Style::default().fg(Color::Cyan).bold(),
    )));
    lines.extend(body_lines[start..end].iter().cloned());
    let footer = if max_scroll == 0 {
        "└ ready".to_string()
    } else {
        format!("└ preview scroll {}", start + 1)
    };
    lines.push(Line::from(Span::styled(
        truncate_preview(&footer, header_width),
        Style::default().fg(Color::DarkGray).dim(),
    )));

    ToolPanelRender { lines, max_scroll }
}

fn render_tool_activity_item(
    body_lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    preview_width: usize,
) {
    match &message.message_type {
        MessageType::ToolCall { name, arguments } => {
            body_lines.push(Line::from(vec![
                Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
                Span::styled("🔧 ", Style::default().fg(Color::Cyan).bold()),
                Span::styled(name.clone(), Style::default().fg(Color::Cyan).bold()),
            ]));
            push_preview_lines(
                body_lines,
                arguments,
                preview_width,
                Style::default().fg(Color::DarkGray).dim(),
                "(no arguments)",
            );
        }
        MessageType::ToolResult {
            name,
            output,
            success,
            duration_ms,
        } => {
            let (icon, color, status) = if *success {
                ("✅ ", Color::Green, "success")
            } else {
                ("❌ ", Color::Red, "error")
            };
            let duration_label = duration_ms
                .map(|ms| format!(" • {ms}ms"))
                .unwrap_or_default();
            body_lines.push(Line::from(vec![
                Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
                Span::styled(icon, Style::default().fg(color).bold()),
                Span::styled(
                    format!("{name} • {status}{duration_label}"),
                    Style::default().fg(color).bold(),
                ),
            ]));
            push_preview_lines(
                body_lines,
                output,
                preview_width,
                Style::default().fg(color).dim(),
                "(empty output)",
            );
        }
        MessageType::Thinking(thoughts) => {
            body_lines.push(Line::from(vec![
                Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
                Span::styled(
                    "💭 thinking",
                    Style::default().fg(Color::DarkGray).dim().italic(),
                ),
            ]));
            push_preview_lines(
                body_lines,
                thoughts,
                preview_width,
                Style::default().fg(Color::DarkGray).dim().italic(),
                "(no reasoning text)",
            );
        }
        _ => {}
    }
}

fn push_preview_lines(
    body_lines: &mut Vec<Line<'static>>,
    text: &str,
    preview_width: usize,
    style: Style,
    empty_label: &str,
) {
    let preview = preview_excerpt(text, preview_width);
    if preview.lines.is_empty() {
        body_lines.push(Line::from(vec![
            Span::styled("│   ", Style::default().fg(Color::DarkGray).dim()),
            Span::styled(empty_label.to_string(), style),
        ]));
        return;
    }

    for line in preview.lines {
        body_lines.push(Line::from(vec![
            Span::styled("│   ", Style::default().fg(Color::DarkGray).dim()),
            Span::styled(line, style),
        ]));
    }

    if preview.truncated {
        body_lines.push(Line::from(vec![
            Span::styled("│   ", Style::default().fg(Color::DarkGray).dim()),
            Span::styled("…", Style::default().fg(Color::DarkGray).dim()),
        ]));
    }
}

struct PreviewExcerpt {
    lines: Vec<String>,
    truncated: bool,
}

fn preview_excerpt(text: &str, preview_width: usize) -> PreviewExcerpt {
    let truncated_bytes = truncate_at_char_boundary(text, TOOL_PANEL_ITEM_MAX_BYTES);
    let bytes_truncated = truncated_bytes.len() < text.len();
    let mut lines = Vec::new();
    let mut remaining = truncated_bytes.lines();

    for line in remaining.by_ref().take(TOOL_PANEL_ITEM_MAX_LINES) {
        lines.push(truncate_preview(line, preview_width));
    }

    PreviewExcerpt {
        lines,
        truncated: bytes_truncated || remaining.next().is_some(),
    }
}

fn truncate_at_char_boundary(text: &str, max_bytes: usize) -> &str {
    if text.len() <= max_bytes {
        return text;
    }

    let mut cutoff = 0;
    for (idx, ch) in text.char_indices() {
        let next = idx + ch.len_utf8();
        if next > max_bytes {
            break;
        }
        cutoff = next;
    }

    &text[..cutoff]
}
