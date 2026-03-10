use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::tui::app::state::App;
use crate::tui::bus_log::render_bus_log;
use crate::tui::chat::message::MessageType;
use crate::tui::color_palette::ColorPalette;
use crate::tui::help::render_help;
use crate::tui::lsp::render_lsp;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ralph_view::render_ralph_view;
use crate::tui::rlm::render_rlm;
use crate::tui::settings::render_settings;
use crate::tui::swarm_view::render_swarm_view;
use crate::tui::symbol_search::render_symbol_search;

fn clamp_scroll(scroll: usize) -> u16 {
    scroll.min(u16::MAX as usize) as u16
}

pub fn ui(f: &mut Frame, app: &mut App) {
    match app.state.view_mode {
        ViewMode::Chat => render_chat_view(f, app),
        ViewMode::Sessions => render_sessions_view(f, app),
        ViewMode::Swarm => render_swarm_view(f, &mut app.state.swarm, f.area()),
        ViewMode::Ralph => render_ralph_view(f, &mut app.state.ralph, f.area()),
        ViewMode::Bus => render_bus_log(f, &mut app.state.bus_log, f.area()),
        ViewMode::Help => render_help(f, f.area()),
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

fn render_chat_view(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    let palette = ColorPalette::marketing();
    let formatter = MessageFormatter::new(chunks[1].width.saturating_sub(4) as usize);

    let title = if app.state.processing {
        "CodeTether TUI — processing"
    } else {
        "CodeTether TUI"
    };

    let session_text = app
        .state
        .session_id
        .as_deref()
        .map(|id| format!("session {id}"))
        .unwrap_or_else(|| "new session".to_string());

    let mode_text = match app.state.input_mode {
        InputMode::Normal => "normal",
        InputMode::Editing => "edit",
        InputMode::Command => "command",
    };

    let header = Paragraph::new(vec![Line::from(vec![
        Span::raw("Status: ").dim(),
        Span::styled(
            app.state.status.clone(),
            Style::default().fg(if app.state.processing {
                Color::Yellow
            } else {
                Color::Green
            }),
        ),
        Span::raw("  •  ").dim(),
        Span::raw(session_text).dim(),
        Span::raw("  •  ").dim(),
        Span::raw(format!("mode {mode_text}")).dim(),
        Span::raw("  •  ").dim(),
        Span::raw("Try /help /sessions /bus /symbols").dim(),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(palette.border))
            .title(title),
    );
    f.render_widget(header, chunks[0]);

    let mut lines: Vec<Line<'static>> = Vec::new();
    if app.state.messages.is_empty() {
        lines.push(Line::from(
            "No messages yet. Type a prompt and press Enter, or use /help.".dim(),
        ));
    } else {
        for msg in &app.state.messages {
            let (label, role) = match msg.message_type {
                MessageType::User => ("You", "user"),
                MessageType::Assistant => ("Assistant", "assistant"),
                MessageType::System => ("System", "system"),
                MessageType::Error => ("Error", "error"),
            };
            let color = palette.get_message_color(role);
            let formatted = formatter.format_content(&msg.content, role);
            let mut iter = formatted.into_iter();
            if let Some(first_line) = iter.next() {
                let mut spans = vec![Span::styled(
                    format!("{label}: "),
                    Style::default().fg(color).bold(),
                )];
                spans.extend(first_line.spans.into_iter());
                lines.push(Line::from(spans));
            }
            for line in iter {
                let mut spans = vec![Span::styled("  ", Style::default().fg(color))];
                spans.extend(line.spans.into_iter());
                lines.push(Line::from(spans));
            }
            lines.push(Line::from(""));
        }
    }

    let chat = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(palette.border))
                .title(format!(
                    "Chat (scroll: {})",
                    app.state.chat_scroll.min(u16::MAX as usize)
                )),
        )
        .wrap(Wrap { trim: false })
        .scroll((clamp_scroll(app.state.chat_scroll), 0));
    f.render_widget(chat, chunks[1]);

    let visible_width = chunks[2].width.saturating_sub(2) as usize;
    app.state.ensure_input_cursor_visible(visible_width);
    let visible_input = app
        .state
        .input
        .chars()
        .skip(app.state.input_scroll)
        .take(visible_width)
        .collect::<String>();

    let input_title = if matches!(app.state.input_mode, InputMode::Command) {
        "Command"
    } else {
        "Input"
    };
    let input = Paragraph::new(visible_input)
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[2]);

    let input_inner_x = chunks[2].x.saturating_add(1);
    let input_inner_y = chunks[2].y.saturating_add(1);
    let cursor_col = app.state.input_cursor.saturating_sub(app.state.input_scroll);
    let cursor_offset = if visible_width == 0 {
        0
    } else {
        cursor_col.min(visible_width.saturating_sub(1)) as u16
    };
    let cursor_x = input_inner_x.saturating_add(cursor_offset);
    f.set_cursor_position((cursor_x, input_inner_y));

    let help = Paragraph::new(
        "Slash commands: /help /sessions /swarm /ralph /bus /settings /lsp /rlm /symbols /chat /new • Enter runs command or sends prompt • Ctrl+T symbols • Esc back • Ctrl+C/Ctrl+Q quit",
    )
    .block(Block::default().borders(Borders::ALL).title("Help"))
    .wrap(Wrap { trim: true });
    f.render_widget(help, chunks[3]);
}

fn render_sessions_view(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let header = Paragraph::new(vec![Line::from(vec![
        Span::raw("Workspace Sessions").cyan().bold(),
        Span::raw("  •  ").dim(),
        Span::raw(app.state.status.clone()).dim(),
    ])])
    .block(Block::default().borders(Borders::ALL).title("Sessions"));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem<'static>> = if app.state.sessions.is_empty() {
        vec![ListItem::new("No workspace sessions found")]
    } else {
        app.state
            .sessions
            .iter()
            .map(|session| {
                let title = session
                    .title
                    .clone()
                    .unwrap_or_else(|| "Untitled session".to_string());
                let summary = format!(
                    "{}  •  {} msgs  •  {}",
                    title,
                    session.message_count,
                    session.updated_at.format("%Y-%m-%d %H:%M")
                );
                ListItem::new(summary)
            })
            .collect()
    };

    let mut state = ListState::default();
    if !app.state.sessions.is_empty() {
        state.select(Some(app.state.selected_session.min(app.state.sessions.len() - 1)));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Available Sessions"))
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold())
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunks[1], &mut state);

    let help = Paragraph::new("↑/↓ select • Enter load selected session • Esc or /chat back to chat • /help for commands")
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: true });
    f.render_widget(help, chunks[2]);
}
