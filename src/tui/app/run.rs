use std::io;
use std::sync::Arc;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use crate::bus::AgentBus;
use crate::provider::{ContentPart, ProviderRegistry, Role};
use crate::session::{list_sessions_for_directory, Session, SessionEvent};
use crate::tool::{lsp::LspTool, Tool};
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ui::main::ui;
use crate::tui::worker_bridge::TuiWorkerBridge;

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, crossterm::event::DisableMouseCapture);
    }
}

fn extract_message_text(content: &[ContentPart]) -> String {
    let mut out = String::new();
    for part in content {
        match part {
            ContentPart::Text { text } | ContentPart::Thinking { text } => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(text);
            }
            ContentPart::ToolCall {
                name, arguments, ..
            } => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format!("Tool call: {name} {arguments}"));
            }
            ContentPart::ToolResult { content, .. } => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(content);
            }
            ContentPart::Image { url, .. } => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format!("[image: {url}]"));
            }
            ContentPart::File { path, .. } => {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&format!("[file: {path}]"));
            }
        }
    }
    out
}

fn sync_messages_from_session(app: &mut App, session: &Session) {
    app.state.messages = session
        .messages
        .iter()
        .filter_map(|msg| {
            let content = extract_message_text(&msg.content);
            if content.trim().is_empty() {
                return None;
            }
            let message_type = match msg.role {
                Role::User => MessageType::User,
                Role::Assistant => MessageType::Assistant,
                Role::System | Role::Tool => MessageType::System,
            };
            Some(ChatMessage::new(message_type, content))
        })
        .collect();
    app.state.scroll_to_bottom();
}

fn truncate_preview(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let preview: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{preview}…")
    } else {
        preview
    }
}

async fn refresh_sessions(app: &mut App, cwd: &std::path::Path) {
    match list_sessions_for_directory(cwd).await {
        Ok(sessions) => {
            app.state.sessions = sessions;
            if app.state.selected_session >= app.state.sessions.len() {
                app.state.selected_session = app.state.sessions.len().saturating_sub(1);
            }
        }
        Err(err) => {
            app.state.status = format!("Failed to list sessions: {err}");
            app.state.sessions.clear();
            app.state.selected_session = 0;
        }
    }
}

fn return_to_chat(app: &mut App) {
    app.state.set_view_mode(ViewMode::Chat);
    app.state.status = "Back to chat".to_string();
}

fn symbol_search_active(app: &App) -> bool {
    !app.state.symbol_search.query.is_empty()
        || !app.state.symbol_search.results.is_empty()
        || app.state.symbol_search.loading
        || app.state.symbol_search.error.is_some()
}

async fn refresh_symbol_search(app: &mut App) {
    app.state.symbol_search.loading = true;
    let result = LspTool::new()
        .execute(serde_json::json!({
            "action": "workspaceSymbol",
            "query": app.state.symbol_search.query.clone(),
        }))
        .await;

    match result {
        Ok(tool_result) if tool_result.success => {
            let results = tool_result
                .output
                .lines()
                .skip(2)
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    Some(crate::tui::symbol_search::SymbolEntry {
                        name: trimmed.to_string(),
                        kind: "Symbol".to_string(),
                        path: std::path::PathBuf::from(trimmed),
                        uri: None,
                        line: None,
                        container: None,
                    })
                })
                .collect();
            app.state.symbol_search.set_results(results);
        }
        Ok(tool_result) => app.state.symbol_search.set_error(tool_result.output),
        Err(err) => app.state.symbol_search.set_error(err.to_string()),
    }
}

async fn handle_slash_command(app: &mut App, cwd: &std::path::Path, command: &str) {
    match command.trim() {
        "/help" | "/?" => app.state.set_view_mode(ViewMode::Help),
        "/sessions" | "/session" => {
            refresh_sessions(app, cwd).await;
            app.state.set_view_mode(ViewMode::Sessions);
            app.state.status = "Session picker".to_string();
        }
        "/swarm" => app.state.set_view_mode(ViewMode::Swarm),
        "/ralph" => app.state.set_view_mode(ViewMode::Ralph),
        "/bus" | "/logs" => app.state.set_view_mode(ViewMode::Bus),
        "/settings" => app.state.set_view_mode(ViewMode::Settings),
        "/lsp" => app.state.set_view_mode(ViewMode::Lsp),
        "/rlm" => app.state.set_view_mode(ViewMode::Rlm),
        "/chat" => return_to_chat(app),
        "/symbols" | "/symbol" => {
            app.state.symbol_search.open();
            app.state.status = "Symbol search".to_string();
        }
        "/new" => {
            app.state.messages.clear();
            app.state.chat_scroll = 0;
            app.state.status = "New chat buffer".to_string();
            app.state.set_view_mode(ViewMode::Chat);
        }
        "/keys" => {
            app.state.status =
                "Slash commands: /help /sessions /swarm /ralph /bus /settings /lsp /rlm /symbols /chat /new"
                    .to_string();
        }
        other => {
            app.state.status = format!("Unknown command: {other}");
        }
    }
}

pub async fn run(project: Option<std::path::PathBuf>) -> anyhow::Result<()> {
    if let Some(project) = project {
        std::env::set_current_dir(&project)?;
    }

    enable_raw_mode()?;
    let _guard = TerminalGuard;

    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let registry = ProviderRegistry::from_vault().await.ok().map(Arc::new);
    let cwd = std::env::current_dir().unwrap_or_default();
    let bus = AgentBus::new().into_arc();
    let mut bus_handle = bus.handle("tui");
    let mut worker_bridge = TuiWorkerBridge::spawn(None, None, None, Arc::clone(&bus))
        .await
        .ok()
        .flatten();

    let mut session = match Session::last_for_directory(Some(&cwd)).await {
        Ok(existing) => existing,
        Err(_) => Session::new().await?,
    };

    let (event_tx, mut event_rx) = mpsc::channel::<SessionEvent>(256);
    let (result_tx, mut result_rx) = mpsc::channel::<anyhow::Result<Session>>(8);

    let mut app = App::default();
    app.state.cwd_display = cwd.display().to_string();
    app.state.session_id = Some(session.id.clone());
    sync_messages_from_session(&mut app, &session);
    refresh_sessions(&mut app, &cwd).await;
    if app.state.messages.is_empty() {
        app.state.status = "Ready — type a message, or use /help for commands.".to_string();
    } else {
        app.state.status = "Loaded previous workspace session".to_string();
    }
    app.state.move_cursor_end();

    loop {
        while let Some(envelope) = bus_handle.try_recv() {
            app.state.bus_log.ingest(&envelope);
        }

        if let Some(bridge) = worker_bridge.as_mut() {
            while let Ok(task) = bridge.task_rx.try_recv() {
                app.state.messages.push(ChatMessage::new(
                    MessageType::System,
                    format!(
                        "Incoming A2A task {} from {}\n{}",
                        task.task_id,
                        task.from_agent.unwrap_or_else(|| "unknown".to_string()),
                        task.message
                    ),
                ));
                app.state.status = format!("Received remote task {}", task.task_id);
                app.state.scroll_to_bottom();
            }
        }

        while let Ok(updated_session) = result_rx.try_recv() {
            match updated_session {
                Ok(updated_session) => {
                    session = updated_session;
                    app.state.session_id = Some(session.id.clone());
                    let _ = session.save().await;
                    refresh_sessions(&mut app, &cwd).await;
                }
                Err(err) => {
                    app.state.processing = false;
                    app.state
                        .messages
                        .push(ChatMessage::new(MessageType::Error, err.to_string()));
                    app.state.status = "Request failed".to_string();
                    app.state.scroll_to_bottom();
                }
            }
        }

        while let Ok(evt) = event_rx.try_recv() {
            match evt {
                SessionEvent::Thinking => {
                    app.state.processing = true;
                    app.state.status = "Thinking…".to_string();
                }
                SessionEvent::ToolCallStart { name, arguments } => {
                    app.state.processing = true;
                    app.state.status = format!("Running tool: {name}");
                    let args_preview = truncate_preview(&arguments, 240);
                    app.state.messages.push(ChatMessage::new(
                        MessageType::System,
                        format!("Tool start: {name}\nArgs: {args_preview}"),
                    ));
                    app.state.scroll_to_bottom();
                }
                SessionEvent::ToolCallComplete {
                    name,
                    output,
                    success,
                } => {
                    let output_preview = truncate_preview(&output, 600);
                    app.state.messages.push(ChatMessage::new(
                        if success {
                            MessageType::System
                        } else {
                            MessageType::Error
                        },
                        format!(
                            "Tool {}: {name}\nOutput: {output_preview}",
                            if success { "complete" } else { "failed" }
                        ),
                    ));
                    app.state.status = format!("Tool finished: {name}");
                    app.state.scroll_to_bottom();
                }
                SessionEvent::TextChunk(_) => {}
                SessionEvent::TextComplete(text) => {
                    app.state
                        .messages
                        .push(ChatMessage::new(MessageType::Assistant, text));
                    app.state.status = "Assistant replied".to_string();
                    app.state.scroll_to_bottom();
                }
                SessionEvent::ThinkingComplete(text) => {
                    if !text.is_empty() {
                        app.state.messages.push(ChatMessage::new(
                            MessageType::System,
                            format!("Thinking:\n{}", truncate_preview(&text, 600)),
                        ));
                        app.state.scroll_to_bottom();
                    }
                }
                SessionEvent::UsageReport {
                    model,
                    prompt_tokens,
                    completion_tokens,
                    duration_ms,
                } => {
                    app.state.status = format!(
                        "Completed with model {model} • {} in / {} out • {} ms",
                        prompt_tokens, completion_tokens, duration_ms
                    );
                }
                SessionEvent::SessionSync(updated) => {
                    session = *updated;
                    app.state.session_id = Some(session.id.clone());
                }
                SessionEvent::Done => {
                    app.state.processing = false;
                    app.state.status = "Ready".to_string();
                }
                SessionEvent::Error(err) => {
                    app.state.processing = false;
                    app.state
                        .messages
                        .push(ChatMessage::new(MessageType::Error, err.clone()));
                    app.state.status = "Error".to_string();
                    app.state.scroll_to_bottom();
                }
            }
        }

        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.state.symbol_search.open();
                        app.state.status = "Symbol search".to_string();
                    }
                    KeyCode::Esc => {
                        if symbol_search_active(&app) {
                            app.state.symbol_search.close();
                            app.state.status = "Closed symbol search".to_string();
                        } else {
                            match app.state.view_mode {
                                ViewMode::Sessions => return_to_chat(&mut app),
                                ViewMode::Swarm if app.state.swarm.detail_mode => app.state.swarm.exit_detail(),
                                ViewMode::Ralph if app.state.ralph.detail_mode => app.state.ralph.exit_detail(),
                                ViewMode::Bus if app.state.bus_log.detail_mode => app.state.bus_log.exit_detail(),
                                ViewMode::Chat => app.state.input_mode = InputMode::Normal,
                                _ => return_to_chat(&mut app),
                            }
                        }
                    }
                    KeyCode::Up if symbol_search_active(&app) => app.state.symbol_search.select_prev(),
                    KeyCode::Up => match app.state.view_mode {
                        ViewMode::Sessions => app.state.sessions_select_prev(),
                        ViewMode::Swarm => {
                            if app.state.swarm.detail_mode {
                                app.state.swarm.detail_scroll_up(1);
                            } else {
                                app.state.swarm.select_prev();
                            }
                        }
                        ViewMode::Ralph => {
                            if app.state.ralph.detail_mode {
                                app.state.ralph.detail_scroll_up(1);
                            } else {
                                app.state.ralph.select_prev();
                            }
                        }
                        ViewMode::Bus => {
                            if app.state.bus_log.detail_mode {
                                app.state.bus_log.detail_scroll_up(1);
                            } else {
                                app.state.bus_log.select_prev();
                            }
                        }
                        _ => app.state.scroll_up(1),
                    },
                    KeyCode::Down if symbol_search_active(&app) => app.state.symbol_search.select_next(),
                    KeyCode::Down => match app.state.view_mode {
                        ViewMode::Sessions => app.state.sessions_select_next(),
                        ViewMode::Swarm => {
                            if app.state.swarm.detail_mode {
                                app.state.swarm.detail_scroll_down(1);
                            } else {
                                app.state.swarm.select_next();
                            }
                        }
                        ViewMode::Ralph => {
                            if app.state.ralph.detail_mode {
                                app.state.ralph.detail_scroll_down(1);
                            } else {
                                app.state.ralph.select_next();
                            }
                        }
                        ViewMode::Bus => {
                            if app.state.bus_log.detail_mode {
                                app.state.bus_log.detail_scroll_down(1);
                            } else {
                                app.state.bus_log.select_next();
                            }
                        }
                        _ => app.state.scroll_down(1),
                    },
                    KeyCode::PageUp => match app.state.view_mode {
                        ViewMode::Swarm if app.state.swarm.detail_mode => {
                            app.state.swarm.detail_scroll_up(10)
                        }
                        ViewMode::Ralph if app.state.ralph.detail_mode => {
                            app.state.ralph.detail_scroll_up(10)
                        }
                        ViewMode::Bus if app.state.bus_log.detail_mode => {
                            app.state.bus_log.detail_scroll_up(10)
                        }
                        ViewMode::Chat => app.state.scroll_up(10),
                        _ => {}
                    },
                    KeyCode::PageDown => match app.state.view_mode {
                        ViewMode::Swarm if app.state.swarm.detail_mode => {
                            app.state.swarm.detail_scroll_down(10)
                        }
                        ViewMode::Ralph if app.state.ralph.detail_mode => {
                            app.state.ralph.detail_scroll_down(10)
                        }
                        ViewMode::Bus if app.state.bus_log.detail_mode => {
                            app.state.bus_log.detail_scroll_down(10)
                        }
                        ViewMode::Chat => app.state.scroll_down(10),
                        _ => {}
                    },
                    KeyCode::Home => {
                        if app.state.view_mode == ViewMode::Chat {
                            app.state.move_cursor_home();
                        }
                    }
                    KeyCode::End => {
                        if app.state.view_mode == ViewMode::Chat {
                            app.state.move_cursor_end();
                        }
                    }
                    KeyCode::Left => {
                        if app.state.view_mode == ViewMode::Chat && !app.state.processing {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                app.state.move_cursor_word_left();
                            } else {
                                app.state.move_cursor_left();
                            }
                        }
                    }
                    KeyCode::Right => {
                        if app.state.view_mode == ViewMode::Chat && !app.state.processing {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                app.state.move_cursor_word_right();
                            } else {
                                app.state.move_cursor_right();
                            }
                        }
                    }
                    KeyCode::Delete => {
                        if app.state.view_mode == ViewMode::Chat && !app.state.processing {
                            app.state.delete_forward();
                        }
                    }
                    KeyCode::Enter if symbol_search_active(&app) => {
                        if let Some(symbol) = app.state.symbol_search.selected_symbol() {
                            app.state.status = format!(
                                "Selected symbol {} {}",
                                symbol.name,
                                symbol
                                    .line
                                    .map(|line| format!("at line {line}"))
                                    .unwrap_or_default()
                            );
                        }
                        app.state.symbol_search.close();
                    }
                    KeyCode::Enter => match app.state.view_mode {
                        ViewMode::Sessions => {
                            if let Some(summary) = app.state.sessions.get(app.state.selected_session) {
                                match Session::load(&summary.id).await {
                                    Ok(loaded) => {
                                        session = loaded;
                                        app.state.session_id = Some(session.id.clone());
                                        sync_messages_from_session(&mut app, &session);
                                        refresh_sessions(&mut app, &cwd).await;
                                        return_to_chat(&mut app);
                                        app.state.status = format!(
                                            "Loaded session {}",
                                            session.title.clone().unwrap_or_else(|| session.id.clone())
                                        );
                                    }
                                    Err(err) => {
                                        app.state.status = format!("Failed to load session: {err}");
                                    }
                                }
                            }
                        }
                        ViewMode::Swarm => app.state.swarm.enter_detail(),
                        ViewMode::Ralph => app.state.ralph.enter_detail(),
                        ViewMode::Bus => app.state.bus_log.enter_detail(),
                        ViewMode::Chat => {
                            if app.state.processing {
                                app.state.status = "Still processing previous request…".to_string();
                            } else {
                                let prompt = app.state.input.trim().to_string();
                                if prompt.starts_with('/') {
                                    handle_slash_command(&mut app, &cwd, &prompt).await;
                                    app.state.input.clear();
                                    app.state.input_cursor = 0;
                                    app.state.input_scroll = 0;
                                } else if !prompt.is_empty() {
                                    app.state
                                        .messages
                                        .push(ChatMessage::new(MessageType::User, prompt.clone()));
                                    app.state.input.clear();
                                    app.state.input_cursor = 0;
                                    app.state.input_scroll = 0;
                                    app.state.processing = true;
                                    app.state.status = "Submitting prompt…".to_string();
                                    app.state.scroll_to_bottom();

                                    if let Some(registry) = &registry {
                                        let mut session_for_task = session.clone();
                                        let event_tx = event_tx.clone();
                                        let result_tx = result_tx.clone();
                                        let registry = Arc::clone(registry);
                                        tokio::spawn(async move {
                                            let result = session_for_task
                                                .prompt_with_events(&prompt, event_tx, registry)
                                                .await
                                                .map(|_| session_for_task);
                                            let _ = result_tx.send(result).await;
                                        });
                                    } else {
                                        app.state.processing = false;
                                        app.state.messages.push(ChatMessage::new(
                                            MessageType::Error,
                                            "No providers available. Configure credentials first (for example: `codetether auth codex` or `codetether auth copilot`).",
                                        ));
                                        app.state.status = "No providers configured".to_string();
                                        app.state.scroll_to_bottom();
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => {
                        if symbol_search_active(&app) {
                            app.state.symbol_search.handle_backspace();
                            refresh_symbol_search(&mut app).await;
                        } else if app.state.view_mode == ViewMode::Chat && !app.state.processing {
                            app.state.delete_backspace();
                            if app.state.input.is_empty() {
                                app.state.input_mode = InputMode::Normal;
                            } else if app.state.input.starts_with('/') {
                                app.state.input_mode = InputMode::Command;
                            }
                        }
                    }
                    KeyCode::Char('g') if app.state.view_mode == ViewMode::Bus => {
                        let len = app.state.bus_log.visible_count();
                        if len > 0 {
                            app.state.bus_log.selected_index = len - 1;
                            app.state.bus_log.auto_scroll = true;
                        }
                    }
                    KeyCode::Char('c') if app.state.view_mode == ViewMode::Bus => {
                        app.state.bus_log.filter.clear();
                    }
                    KeyCode::Char('/') if app.state.view_mode == ViewMode::Bus => {
                        app.state.status =
                            "Bus filter editing not yet interactive; press c to clear current filter"
                                .to_string();
                    }
                    KeyCode::Char(c) => {
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT)
                            && symbol_search_active(&app)
                        {
                            app.state.symbol_search.handle_char(c);
                            refresh_symbol_search(&mut app).await;
                        } else if app.state.view_mode == ViewMode::Chat
                            && !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT)
                            && !app.state.processing
                        {
                            app.state.input_mode = if app.state.input.is_empty() && c == '/' {
                                InputMode::Command
                            } else if app.state.input.starts_with('/') || c == '/' {
                                InputMode::Command
                            } else {
                                InputMode::Editing
                            };
                            app.state.insert_char(c);
                        }
                    }
                    _ => {}
                },
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    terminal.show_cursor()?;
    Ok(())
}
