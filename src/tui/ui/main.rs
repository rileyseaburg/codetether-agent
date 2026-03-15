use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Wrap,
    },
};

use unicode_width::UnicodeWidthStr;

use crate::tui::app::state::App;
use crate::tui::app::text::truncate_preview;
use crate::tui::bus_log::{ProtocolSummary, render_bus_log_with_summary};
use crate::tui::chat::message::MessageType;
use crate::tui::color_palette::ColorPalette;
use crate::tui::lsp::render_lsp;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ralph_view::render_ralph_view;
use crate::tui::rlm::render_rlm;
use crate::tui::settings::render_settings;
use crate::tui::swarm_view::render_swarm_view;
use crate::tui::symbol_search::render_symbol_search;

/// Max lines to show for tool call arguments preview.
const TOOL_ARGS_PREVIEW_MAX_LINES: usize = 10;
/// Max lines to show for tool result output preview.
const TOOL_OUTPUT_PREVIEW_MAX_LINES: usize = 5;

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
        ViewMode::Settings => render_settings(f, f.area(), &app.state.status),
        ViewMode::Lsp => render_lsp(f, f.area(), &app.state.cwd_display, &app.state.status),
        ViewMode::Rlm => render_rlm(
            f,
            f.area(),
            &app.state.cwd_display,
            &app.state.status,
            app.state.sessions.len(),
            app.state.selected_session,
        ),
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
    if app.state.messages.is_empty() {
        lines.push(Line::from(
            "No messages yet. Type a prompt and press Enter, or use /help.".dim(),
        ));
    } else {
        for (idx, msg) in app.state.messages.iter().enumerate() {
            let (label, role, icon, color) = match &msg.message_type {
                MessageType::User => ("user", "user", "▸ ", palette.get_message_color("user")),
                MessageType::Assistant => (
                    "assistant",
                    "assistant",
                    "◆ ",
                    palette.get_message_color("assistant"),
                ),
                MessageType::System => (
                    "system",
                    "system",
                    "⚙ ",
                    palette.get_message_color("system"),
                ),
                MessageType::Error => ("error", "error", "✖ ", palette.get_message_color("error")),
                MessageType::ToolCall { name, .. } => {
                    // Render ToolCall with wrench icon and pipe-prefixed args
                    if idx > 0 {
                        lines.push(Line::from(Span::styled(
                            "·".repeat(separator_width),
                            Style::default().fg(Color::DarkGray).dim(),
                        )));
                    }
                    let timestamp = format_timestamp(msg.timestamp);
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("[{timestamp}] "),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                        Span::styled("🔧 ", Style::default().fg(Color::Cyan).bold()),
                        Span::styled(
                            format!("tool call: {name}"),
                            Style::default().fg(Color::Cyan).bold(),
                        ),
                    ]));
                    for line in msg.content.lines().take(TOOL_ARGS_PREVIEW_MAX_LINES) {
                        lines.push(Line::from(vec![
                            Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
                            Span::styled(
                                line.to_string(),
                                Style::default().fg(Color::DarkGray).dim(),
                            ),
                        ]));
                    }
                    lines.push(Line::from(""));
                    lines.push(Line::from(""));
                    continue;
                }
                MessageType::ToolResult {
                    name,
                    success,
                    duration_ms,
                    ..
                } => {
                    if idx > 0 {
                        lines.push(Line::from(Span::styled(
                            "·".repeat(separator_width),
                            Style::default().fg(Color::DarkGray).dim(),
                        )));
                    }
                    let timestamp = format_timestamp(msg.timestamp);
                    let (result_icon, result_color) = if *success {
                        ("✅ ", Color::Green)
                    } else {
                        ("❌ ", Color::Red)
                    };
                    let duration_label = duration_ms
                        .map(|ms| format!(" ({ms}ms)"))
                        .unwrap_or_default();
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("[{timestamp}] "),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                        Span::styled(result_icon, Style::default().fg(result_color).bold()),
                        Span::styled(
                            format!("tool result: {name}{duration_label}"),
                            Style::default().fg(result_color).bold(),
                        ),
                    ]));
                    for line in msg.content.lines().take(TOOL_OUTPUT_PREVIEW_MAX_LINES) {
                        lines.push(Line::from(vec![
                            Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
                            Span::styled(line.to_string(), Style::default().fg(result_color).dim()),
                        ]));
                    }
                    lines.push(Line::from(""));
                    continue;
                }
                MessageType::Thinking(_) => {
                    if idx > 0 {
                        lines.push(Line::from(Span::styled(
                            "·".repeat(separator_width),
                            Style::default().fg(Color::DarkGray).dim(),
                        )));
                    }
                    let timestamp = format_timestamp(msg.timestamp);
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("[{timestamp}] "),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                        Span::styled(
                            "💭 thinking",
                            Style::default().fg(Color::DarkGray).italic().dim(),
                        ),
                    ]));
                    for text_line in msg.content.lines().take(5) {
                        lines.push(Line::from(Span::styled(
                            format!("  {text_line}"),
                            Style::default().fg(Color::DarkGray).dim().italic(),
                        )));
                    }
                    lines.push(Line::from(""));
                    continue;
                }
                MessageType::Image { .. } => {
                    if idx > 0 {
                        lines.push(Line::from(Span::styled(
                            "·".repeat(separator_width),
                            Style::default().fg(Color::DarkGray).dim(),
                        )));
                    }
                    let timestamp = format_timestamp(msg.timestamp);
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
                        format!("  {}", truncate_preview(&msg.content, 120)),
                        Style::default().fg(Color::Cyan).dim(),
                    )));
                    lines.push(Line::from(""));
                    continue;
                }
                MessageType::File { path, size } => {
                    if idx > 0 {
                        lines.push(Line::from(Span::styled(
                            "·".repeat(separator_width),
                            Style::default().fg(Color::DarkGray).dim(),
                        )));
                    }
                    let timestamp = format_timestamp(msg.timestamp);
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
                    lines.push(Line::from(""));
                    continue;
                }
            };
            if idx > 0 {
                let sep = if role == "system" || role == "error" {
                    "·"
                } else {
                    "─"
                };
                lines.push(Line::from(Span::styled(
                    sep.repeat(separator_width),
                    Style::default().fg(Color::DarkGray).dim(),
                )));
            }
            let timestamp = format_timestamp(msg.timestamp);
            lines.push(Line::from(vec![
                Span::styled(
                    format!("[{timestamp}] "),
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::DIM),
                ),
                Span::styled(icon, Style::default().fg(color).bold()),
                Span::styled(label, Style::default().fg(color).bold()),
            ]));
            let formatted = formatter.format_content(&msg.content, role);
            for line in formatted {
                let mut spans = vec![Span::styled("  ", Style::default().fg(color))];
                spans.extend(line.spans.into_iter());
                lines.push(Line::from(spans));
            }
            lines.push(Line::from(""));
        }
    }

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
    // SCROLL_BOTTOM means "follow latest" — jump to end.
    let scroll = if app.state.chat_scroll >= SCROLL_BOTTOM {
        max_scroll
    } else {
        app.state.chat_scroll.min(max_scroll)
    };

    let chat = Paragraph::new(lines)
        .block(messages_block)
        .wrap(Wrap { trim: false })
        .scroll((0, scroll as u16));
    f.render_widget(chat, chunks[0]);

    // Input area: show all lines of multi-line input with wrapping.
    let input_title = if app.state.processing {
        " Message (Processing...) "
    } else if matches!(app.state.input_mode, InputMode::Command) {
        " Command (/ for commands, Tab to autocomplete) "
    } else {
        " Message (Enter to send, / for commands) "
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
        chunks[1].x.saturating_add(chunks[1].width.saturating_sub(2)),
    );
    let cursor_y = input_inner_y
        .saturating_add(cursor_row as u16)
        .min(chunks[1].y.saturating_add(chunks[1].height.saturating_sub(2)));
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

    let status = Paragraph::new(Line::from(vec![
        Span::styled(" CHAT ", Style::default().fg(Color::Black).bg(Color::Cyan)),
        Span::raw(" | "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(": Help | "),
        Span::styled("Tab", Style::default().fg(Color::Yellow)),
        Span::raw(": Complete | "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
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
        bus_status_badge_span(app),
        Span::raw(" | "),
        Span::styled(
            app.state.status.clone(),
            Style::default().fg(if app.state.processing {
                Color::Yellow
            } else {
                Color::Green
            }),
        ),
    ]));
    f.render_widget(status, chunks[help_index]);

    crate::tui::help::render_help_overlay_if_needed(f, &mut app.state);
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
