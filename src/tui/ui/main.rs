use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use unicode_width::UnicodeWidthStr;

use crate::tui::app::state::App;
use crate::tui::app::text::truncate_preview;
use crate::tui::bus_log::{ProtocolSummary, render_bus_log_with_summary};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::color_palette::ColorPalette;
use crate::tui::latency::render_latency;
use crate::tui::lsp::render_lsp;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ralph_view::render_ralph_view;
use crate::tui::rlm::render_rlm;
use crate::tui::settings::render_settings;
use crate::tui::swarm_view::render_swarm_view;
use crate::tui::symbol_search::render_symbol_search;
use crate::tui::theme::Theme;
use crate::tui::theme_utils::validate_theme;
use crate::tui::token_display::TokenDisplay;

/// Max visible lines inside the compact tool panel.
const TOOL_PANEL_VISIBLE_LINES: usize = 6;
/// Max preview lines captured for a single tool activity item.
const TOOL_PANEL_ITEM_MAX_LINES: usize = 18;
/// Max bytes processed for a single tool activity preview.
const TOOL_PANEL_ITEM_MAX_BYTES: usize = 6_000;

/// Sentinel value meaning "follow the latest message" (scroll to bottom).
const SCROLL_BOTTOM: usize = 1_000_000;

pub fn ui(f: &mut Frame, app: &mut App, session: &crate::session::Session) {
    match app.state.view_mode {
        ViewMode::Chat => render_chat_view(f, app, session),
        ViewMode::Sessions => render_sessions_view(f, app),
        ViewMode::Swarm => render_swarm_view(f, &mut app.state.swarm, f.area()),
        ViewMode::Ralph => render_ralph_view(f, &mut app.state.ralph, f.area()),
        ViewMode::Bus => {
            let mut registered_agents = app
                .state
                .worker_bridge_registered_agents
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            registered_agents.sort();
            let summary = ProtocolSummary {
                cwd_display: app.state.cwd_display.clone(),
                worker_id: app.state.worker_id.clone(),
                worker_name: app.state.worker_name.clone(),
                a2a_connected: app.state.a2a_connected,
                processing: app.state.worker_bridge_processing_state,
                registered_agents,
                queued_tasks: app.state.worker_task_queue.len(),
                recent_task: app.state.recent_tasks.last().cloned(),
            };
            render_bus_log_with_summary(f, &mut app.state.bus_log, f.area(), Some(summary))
        }
        ViewMode::Model => crate::tui::model_picker::render_model_picker(f, f.area(), app, session),
        ViewMode::Settings => render_settings(f, f.area(), &app.state),
        ViewMode::Lsp => render_lsp(f, f.area(), &app.state.cwd_display, &app.state.status),
        ViewMode::Rlm => render_rlm(
            f,
            f.area(),
            &app.state.cwd_display,
            &app.state.status,
            app.state.sessions.len(),
            app.state.selected_session,
        ),
        ViewMode::Latency => render_latency(f, f.area(), app),
    }

    if app.state.symbol_search.loading
        || !app.state.symbol_search.query.is_empty()
        || !app.state.symbol_search.results.is_empty()
        || app.state.symbol_search.error.is_some()
    {
        render_symbol_search(f, &mut app.state.symbol_search, f.area());
    }
}

fn render_chat_view(f: &mut Frame, app: &mut App, session: &crate::session::Session) {
    let area = f.area();
    let suggestions_visible = app.state.slash_suggestions_visible();
    let input_lines_count = app.state.input.lines().count().max(1);
    let input_height = (input_lines_count as u16 + 2).clamp(3, 6); // +2 for borders
    let constraints: &[Constraint] = if suggestions_visible {
        &[
            Constraint::Min(8),
            Constraint::Length(input_height),
            Constraint::Length(5),
            Constraint::Length(1),
        ]
    } else {
        &[
            Constraint::Min(8),
            Constraint::Length(input_height),
            Constraint::Length(1),
        ]
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let palette = ColorPalette::marketing();
    let formatter = MessageFormatter::new(chunks[0].width.saturating_sub(4) as usize);

    let mut lines: Vec<Line<'static>> = Vec::new();
    let separator_width = chunks[0].width.saturating_sub(4).min(60) as usize;
    let panel_width = chunks[0].width.saturating_sub(6) as usize;
    let entries = build_render_entries(&app.state.messages);
    let mut tool_preview_max_scroll = 0;
    if entries.is_empty() {
        lines.push(Line::from(
            "No messages yet. Type a prompt and press Enter, or use /help.".dim(),
        ));
    } else {
        for (idx, entry) in entries.iter().enumerate() {
            if idx > 0 {
                let sep = separator_pattern(entry);
                lines.push(Line::from(Span::styled(
                    sep.repeat(separator_width),
                    Style::default().fg(Color::DarkGray).dim(),
                )));
            }
            if !entry.tool_activity.is_empty() {
                let panel = build_tool_activity_panel(
                    &entry.tool_activity,
                    app.state.tool_preview_scroll,
                    panel_width,
                );
                tool_preview_max_scroll = tool_preview_max_scroll.max(panel.max_scroll);
                lines.extend(panel.lines);
                if entry.message.is_some() {
                    lines.push(Line::from(""));
                }
            }
            if let Some(message) = entry.message {
                render_chat_message(&mut lines, message, &formatter, &palette);
                lines.push(Line::from(""));
            }
        }
    }
    app.state
        .set_tool_preview_max_scroll(tool_preview_max_scroll);

    // Streaming text preview (dim, shown during processing)
    if app.state.processing && !app.state.streaming_text.is_empty() {
        lines.push(Line::from(Span::styled(
            "─".repeat(separator_width.min(40)),
            Style::default().fg(Color::DarkGray).dim(),
        )));
        lines.push(Line::from(Span::styled(
            "▸ streaming...",
            Style::default().fg(Color::Yellow).dim(),
        )));
        // Show last few lines of streaming text
        let stream_lines: Vec<&str> = app.state.streaming_text.lines().collect();
        let start = stream_lines.len().saturating_sub(6);
        for sl in &stream_lines[start..] {
            lines.push(Line::from(Span::styled(
                format!("  {sl}"),
                Style::default().fg(Color::DarkGray).dim(),
            )));
        }
    }

    let session_label = app
        .state
        .session_id
        .as_deref()
        .map(|id| {
            if id.len() > 18 {
                format!("{}…", &id[..18])
            } else {
                id.to_string()
            }
        })
        .unwrap_or_else(|| "new".to_string());
    let model_label = session
        .metadata
        .model
        .clone()
        .or_else(|| session_model_label(&app.state))
        .unwrap_or_else(|| "auto".to_string());
    let message_title =
        format!(" CodeTether Agent [main] model:{model_label} session:{session_label} ");
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.border))
        .title(message_title);

    // Calculate total rendered height accounting for line wrapping so that
    // scroll bounds are accurate even when long lines wrap to multiple rows.
    let content_width = chunks[0].width.saturating_sub(2) as usize; // minus borders
    let total_rendered_height: usize = lines
        .iter()
        .map(|line| {
            let w = line.width();
            if w == 0 || content_width == 0 {
                1
            } else {
                w.div_ceil(content_width)
            }
        })
        .sum();
    let visible_height = chunks[0].height.saturating_sub(2) as usize;
    let max_scroll = total_rendered_height.saturating_sub(visible_height);
    app.state.set_chat_max_scroll(max_scroll);
    // SCROLL_BOTTOM means "follow latest" — jump to end.
    let scroll = if app.state.chat_scroll >= SCROLL_BOTTOM {
        max_scroll
    } else {
        app.state.chat_scroll.min(max_scroll)
    };

    let chat = Paragraph::new(lines)
        .block(messages_block)
        .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0));
    f.render_widget(chat, chunks[0]);

    // Input area: show all lines of multi-line input with wrapping.
    let pending_images = app.state.pending_images.len();
    let token_display = TokenDisplay::new();
    let validated_theme = validate_theme(&Theme::default());
    let token_status_line = token_display.create_status_bar(&validated_theme);
    let attachment_suffix = if pending_images > 0 {
        format!(" | 📷 {pending_images} attached ")
    } else {
        String::new()
    };
    let input_title = if app.state.processing {
        format!(" Message (Processing...){attachment_suffix}")
    } else if matches!(app.state.input_mode, InputMode::Command) {
        format!(" Command (/ for commands, Tab to autocomplete){attachment_suffix}")
    } else {
        format!(" Message (Enter to send, Ctrl+V image){attachment_suffix}")
    };
    let input_border = if app.state.processing {
        Color::Yellow
    } else if matches!(app.state.input_mode, InputMode::Command) {
        Color::Magenta
    } else {
        palette.border
    };
    let input = Paragraph::new(app.state.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(input_border))
                .title(input_title),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(input, chunks[1]);

    // Place cursor at the correct position within the input area.
    let input_inner_x = chunks[1].x.saturating_add(1);
    let input_inner_y = chunks[1].y.saturating_add(1);
    // Determine which visual line the cursor is on and its column offset.
    let input_text = &app.state.input;
    let cursor_byte = input_text
        .char_indices()
        .nth(app.state.input_cursor)
        .map(|(i, _)| i)
        .unwrap_or(input_text.len());
    let text_before_cursor = &input_text[..cursor_byte];
    let cursor_row = text_before_cursor.matches('\n').count();
    let last_newline = text_before_cursor.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let cursor_col_display = text_before_cursor[last_newline..].width();
    let cursor_x = input_inner_x.saturating_add(cursor_col_display as u16).min(
        chunks[1]
            .x
            .saturating_add(chunks[1].width.saturating_sub(2)),
    );
    let cursor_y = input_inner_y.saturating_add(cursor_row as u16).min(
        chunks[1]
            .y
            .saturating_add(chunks[1].height.saturating_sub(2)),
    );
    f.set_cursor_position((cursor_x, cursor_y));

    let help_index = if suggestions_visible { 3 } else { 2 };

    if suggestions_visible {
        let items: Vec<ListItem<'static>> = app
            .state
            .slash_suggestions
            .iter()
            .enumerate()
            .map(|(idx, cmd)| {
                let prefix = if idx == app.state.selected_slash_suggestion {
                    "▶ "
                } else {
                    "  "
                };
                ListItem::new(Line::from(vec![
                    Span::raw(prefix),
                    Span::styled(cmd.clone(), Style::default().fg(Color::Cyan).bold()),
                ]))
            })
            .collect();

        let mut list_state = ListState::default();
        if !app.state.slash_suggestions.is_empty() {
            list_state.select(Some(app.state.selected_slash_suggestion));
        }

        let suggestions = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Commands "))
            .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold());
        f.render_stateful_widget(suggestions, chunks[2], &mut list_state);
    }

    let mut status_spans = vec![
        Span::styled(" CHAT ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" | "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(": Help | "),
        Span::styled("Tab", Style::default().fg(Color::Yellow)),
        Span::raw(": Complete | "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Scroll | "),
        Span::styled("Shift+↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Tools | "),
        Span::styled("Ctrl+↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": History | "),
        Span::styled("Ctrl+T", Style::default().fg(Color::Yellow)),
        Span::raw(": Symbols | "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back | "),
        Span::styled("Ctrl+C", Style::default().fg(Color::Yellow)),
        Span::raw(": Quit | "),
        Span::styled("session", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(session_label.clone(), Style::default().fg(Color::Cyan)),
        Span::raw(" | "),
        Span::styled("auto-apply", Style::default().fg(Color::DarkGray)),
        Span::raw(": "),
        Span::styled(
            if app.state.auto_apply_edits {
                "ON"
            } else {
                "OFF"
            },
            Style::default().fg(if app.state.auto_apply_edits {
                Color::Green
            } else {
                Color::Yellow
            }),
        ),
        Span::raw(" | "),
        bus_status_badge_span(app),
    ];
    if pending_images > 0 {
        status_spans.push(Span::raw(" | "));
        status_spans.push(Span::styled(
            format!(" 📷 {pending_images} attached "),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
    }
    if let Some(latency_spans) = latency_badge_spans(app) {
        status_spans.push(Span::raw(" | "));
        status_spans.extend(latency_spans);
    }
    status_spans.push(Span::raw(" | "));
    status_spans.extend(token_status_line.spans);
    status_spans.extend([
        Span::raw(" | "),
        Span::styled(
            app.state.status.clone(),
            Style::default().fg(if app.state.processing {
                Color::Yellow
            } else {
                Color::Green
            }),
        ),
    ]);
    let status = Paragraph::new(Line::from(status_spans));
    f.render_widget(status, chunks[help_index]);

    crate::tui::help::render_help_overlay_if_needed(f, &mut app.state);
}

struct RenderEntry<'a> {
    tool_activity: Vec<&'a ChatMessage>,
    message: Option<&'a ChatMessage>,
}

struct ToolPanelRender {
    lines: Vec<Line<'static>>,
    max_scroll: usize,
}

fn build_render_entries(messages: &[ChatMessage]) -> Vec<RenderEntry<'_>> {
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

fn is_tool_activity(message_type: &MessageType) -> bool {
    matches!(
        message_type,
        MessageType::ToolCall { .. } | MessageType::ToolResult { .. } | MessageType::Thinking(_)
    )
}

fn separator_pattern(entry: &RenderEntry<'_>) -> &'static str {
    match entry.message.map(|message| &message.message_type) {
        Some(MessageType::System | MessageType::Error) | None => "·",
        _ => "─",
    }
}

fn render_chat_message(
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
                        .add_modifier(Modifier::DIM),
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
                        .add_modifier(Modifier::DIM),
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
    lines.push(Line::from(vec![
        Span::styled(
            format!("[{timestamp}] "),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        ),
        Span::styled(icon.to_string(), Style::default().fg(color).bold()),
        Span::styled(label.to_string(), Style::default().fg(color).bold()),
    ]));
    let formatted = formatter.format_content(&message.content, label);
    for line in formatted {
        let mut spans = vec![Span::styled("  ", Style::default().fg(color))];
        spans.extend(line.spans.into_iter());
        lines.push(Line::from(spans));
    }
}

fn build_tool_activity_panel(
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

fn format_timestamp(timestamp: SystemTime) -> String {
    let secs = timestamp
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    let day = secs % 86_400;
    let hour = day / 3_600;
    let minute = (day % 3_600) / 60;
    let second = day % 60;
    format!("{hour:02}:{minute:02}:{second:02}")
}

fn bus_status_label_and_color(app: &App) -> (String, Color) {
    let connected = app.state.a2a_connected;
    let agents = app.state.worker_bridge_registered_agents.len();
    let queued = app.state.worker_task_queue.len();
    let processing = app
        .state
        .worker_bridge_processing_state
        .unwrap_or(app.state.processing);

    if connected || agents > 0 || queued > 0 {
        let state = if processing { "active" } else { "idle" };
        (
            format!("BUS {state} ({agents}ag/{queued}q)"),
            if processing {
                Color::Green
            } else {
                Color::Yellow
            },
        )
    } else {
        ("BUS offline".to_string(), Color::DarkGray)
    }
}

fn bus_status_badge_span(app: &App) -> Span<'static> {
    let (label, color) = bus_status_label_and_color(app);
    Span::styled(
        format!(" {label} "),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

fn latency_badge_spans(app: &App) -> Option<Vec<Span<'static>>> {
    let mut spans = Vec::new();

    if app.state.processing {
        spans.push(request_timing_span(
            "TTFT",
            app.state.current_request_first_token_ms,
            Color::Cyan,
            true,
        ));
        if let Some(elapsed_ms) = app.state.current_request_elapsed_ms() {
            spans.push(Span::raw(" "));
            spans.push(Span::styled(
                format!(" ELAPSED {} ", format_duration_ms(elapsed_ms)),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        return Some(spans);
    }

    if app.state.last_request_first_token_ms.is_none()
        && app.state.last_request_last_token_ms.is_none()
    {
        return None;
    }

    spans.push(request_timing_span(
        "TTFT",
        app.state.last_request_first_token_ms,
        Color::Cyan,
        false,
    ));
    spans.push(Span::raw(" "));
    spans.push(request_timing_span(
        "TTL",
        app.state.last_request_last_token_ms,
        Color::Green,
        false,
    ));
    Some(spans)
}

fn request_timing_span(
    label: &str,
    duration_ms: Option<u64>,
    color: Color,
    emphasize: bool,
) -> Span<'static> {
    let display = duration_ms
        .map(format_duration_ms)
        .unwrap_or_else(|| "…".to_string());
    let mut style = Style::default().fg(color);
    if emphasize {
        style = style.add_modifier(Modifier::BOLD);
    }
    Span::styled(format!(" {label} {display} "), style)
}

fn format_duration_ms(duration_ms: u64) -> String {
    if duration_ms >= 60_000 {
        format!(
            "{}m{:02}s",
            duration_ms / 60_000,
            (duration_ms % 60_000) / 1_000
        )
    } else if duration_ms >= 1_000 {
        format!("{:.1}s", duration_ms as f64 / 1_000.0)
    } else {
        format!("{duration_ms}ms")
    }
}

fn session_model_label(state: &crate::tui::app::state::AppState) -> Option<String> {
    if state.processing {
        Some("processing".to_string())
    } else {
        None
    }
}

fn render_sessions_view(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    let filter_display = if app.state.session_filter.is_empty() {
        String::new()
    } else {
        format!(" [filter: {}]", app.state.session_filter)
    };

    let header = Paragraph::new(vec![Line::from(vec![
        Span::raw(" Session picker ").black().on_cyan(),
        Span::raw(" "),
        Span::raw(app.state.status.clone()).dim(),
    ])])
    .block(Block::default().borders(Borders::ALL).title(format!(
        " Sessions (↑↓ navigate, Enter load, Esc cancel){} ",
        filter_display
    )));
    f.render_widget(header, chunks[0]);

    let filtered = app.state.filtered_sessions();
    let items: Vec<ListItem<'static>> = if filtered.is_empty() {
        if app.state.session_filter.is_empty() {
            vec![ListItem::new("No workspace sessions found")]
        } else {
            vec![ListItem::new(format!(
                "No sessions matching '{}'",
                app.state.session_filter
            ))]
        }
    } else {
        filtered
            .iter()
            .map(|(_, session)| {
                let title = session
                    .title
                    .clone()
                    .unwrap_or_else(|| "Untitled session".to_string());
                let active_marker = if app.state.session_id.as_deref() == Some(session.id.as_str())
                {
                    " ●"
                } else {
                    ""
                };
                let summary = format!(
                    "{}{}  •  {} msgs  •  {}",
                    title,
                    active_marker,
                    session.message_count,
                    session.updated_at.format("%Y-%m-%d %H:%M")
                );
                ListItem::new(summary)
            })
            .collect()
    };

    let mut state = ListState::default();
    if !filtered.is_empty() {
        state.select(Some(app.state.selected_session.min(filtered.len() - 1)));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Available Sessions "),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold())
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, chunks[1], &mut state);

    let help = Paragraph::new(Line::from(vec![
        Span::styled(
            " SESSION PICKER ",
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Nav "),
        Span::styled("Enter", Style::default().fg(Color::Yellow)),
        Span::raw(": Load "),
        Span::styled("Type", Style::default().fg(Color::Yellow)),
        Span::raw(": Filter "),
        Span::styled("Backspace", Style::default().fg(Color::Yellow)),
        Span::raw(": Edit "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Cancel | "),
        bus_status_badge_span(app),
    ]));
    f.render_widget(help, chunks[2]);
}
