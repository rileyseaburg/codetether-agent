//! Terminal User Interface
//!
//! Interactive TUI using Ratatui

pub mod message_formatter;
pub mod swarm_view;
pub mod theme;
pub mod theme_utils;
pub mod token_display;

/// Sentinel value meaning "scroll to bottom"
const SCROLL_BOTTOM: usize = 1_000_000;

use crate::config::Config;
use crate::provider::{ContentPart, Role};
use crate::session::{Session, SessionEvent, SessionSummary, list_sessions};
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor};
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::swarm_view::{SwarmEvent, SwarmViewState, render_swarm_view};
use crate::tui::theme::Theme;
use crate::tui::token_display::TokenDisplay;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
};
use std::io;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Run the TUI
pub async fn run(project: Option<PathBuf>) -> Result<()> {
    // Change to project directory if specified
    if let Some(dir) = project {
        std::env::set_current_dir(&dir)?;
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let result = run_app(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

/// Message type for chat display
#[derive(Debug, Clone)]
enum MessageType {
    Text(String),
    ToolCall { name: String, arguments: String },
    ToolResult { name: String, output: String },
}

/// View mode for the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Chat,
    Swarm,
    SessionPicker,
    ModelPicker,
}

/// Application state
struct App {
    input: String,
    cursor_position: usize,
    messages: Vec<ChatMessage>,
    current_agent: String,
    scroll: usize,
    show_help: bool,
    command_history: Vec<String>,
    history_index: Option<usize>,
    session: Option<Session>,
    is_processing: bool,
    processing_message: Option<String>,
    current_tool: Option<String>,
    response_rx: Option<mpsc::Receiver<SessionEvent>>,
    // Swarm mode state
    view_mode: ViewMode,
    swarm_state: SwarmViewState,
    swarm_rx: Option<mpsc::Receiver<SwarmEvent>>,
    // Session picker state
    session_picker_list: Vec<SessionSummary>,
    session_picker_selected: usize,
    // Model picker state
    model_picker_list: Vec<(String, String)>, // (display label, provider/model value)
    model_picker_selected: usize,
    model_picker_filter: String,
    active_model: Option<String>,
    // Cached max scroll for key handlers
    last_max_scroll: usize,
}

#[allow(dead_code)]
struct ChatMessage {
    role: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

impl ChatMessage {
    fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        Self {
            role: role.into(),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            message_type: MessageType::Text(content.clone()),
            content,
        }
    }

    fn with_message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            messages: vec![
                ChatMessage::new("system", "Welcome to CodeTether Agent! Press ? for help."),
                ChatMessage::new(
                    "assistant",
                    "Quick start:\n• Type a message to chat with the AI\n• /swarm <task> - parallel execution\n• /sessions - pick a session to resume\n• /resume - continue last session\n• Tab - switch agents | ? - help",
                ),
            ],
            current_agent: "build".to_string(),
            scroll: 0,
            show_help: false,
            command_history: Vec::new(),
            history_index: None,
            session: None,
            is_processing: false,
            processing_message: None,
            current_tool: None,
            response_rx: None,
            view_mode: ViewMode::Chat,
            swarm_state: SwarmViewState::new(),
            swarm_rx: None,
            session_picker_list: Vec::new(),
            session_picker_selected: 0,
            model_picker_list: Vec::new(),
            model_picker_selected: 0,
            model_picker_filter: String::new(),
            active_model: None,
            last_max_scroll: 0,
        }
    }

    async fn submit_message(&mut self, config: &Config) {
        if self.input.is_empty() {
            return;
        }

        let message = std::mem::take(&mut self.input);
        self.cursor_position = 0;

        // Save to command history
        if !message.trim().is_empty() {
            self.command_history.push(message.clone());
            self.history_index = None;
        }

        // Check for /swarm command
        if message.trim().starts_with("/swarm ") {
            let task = message
                .trim()
                .strip_prefix("/swarm ")
                .unwrap_or("")
                .to_string();
            if task.is_empty() {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Usage: /swarm <task description>",
                ));
                return;
            }
            self.start_swarm_execution(task, config).await;
            return;
        }

        // Check for /view command to toggle views
        if message.trim() == "/view" || message.trim() == "/swarm" {
            self.view_mode = match self.view_mode {
                ViewMode::Chat | ViewMode::SessionPicker | ViewMode::ModelPicker => ViewMode::Swarm,
                ViewMode::Swarm => ViewMode::Chat,
            };
            return;
        }

        // Check for /sessions command - open session picker
        if message.trim() == "/sessions" {
            match list_sessions().await {
                Ok(sessions) => {
                    if sessions.is_empty() {
                        self.messages
                            .push(ChatMessage::new("system", "No saved sessions found."));
                    } else {
                        self.session_picker_list = sessions.into_iter().take(10).collect();
                        self.session_picker_selected = 0;
                        self.view_mode = ViewMode::SessionPicker;
                    }
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to list sessions: {}", e),
                    ));
                }
            }
            return;
        }

        // Check for /resume command to load a session
        if message.trim() == "/resume" || message.trim().starts_with("/resume ") {
            let session_id = message
                .trim()
                .strip_prefix("/resume")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());
            let loaded = if let Some(id) = session_id {
                Session::load(id).await
            } else {
                Session::last().await
            };

            match loaded {
                Ok(session) => {
                    // Convert session messages to chat messages
                    self.messages.clear();
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Resumed session: {}\nCreated: {}\n{} messages loaded",
                            session.title.as_deref().unwrap_or("(untitled)"),
                            session.created_at.format("%Y-%m-%d %H:%M"),
                            session.messages.len()
                        ),
                    ));

                    for msg in &session.messages {
                        let role_str = match msg.role {
                            Role::System => "system",
                            Role::User => "user",
                            Role::Assistant => "assistant",
                            Role::Tool => "tool",
                        };

                        // Extract text content
                        let content: String = msg
                            .content
                            .iter()
                            .filter_map(|part| match part {
                                ContentPart::Text { text } => Some(text.clone()),
                                ContentPart::ToolCall {
                                    name, arguments, ..
                                } => Some(format!("[Tool: {}]\n{}", name, arguments)),
                                ContentPart::ToolResult { content, .. } => {
                                    let truncated = if content.len() > 500 {
                                        format!("{}...", &content[..497])
                                    } else {
                                        content.clone()
                                    };
                                    Some(format!("[Result]\n{}", truncated))
                                }
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("\n");

                        if !content.is_empty() {
                            self.messages.push(ChatMessage::new(role_str, content));
                        }
                    }

                    self.current_agent = session.agent.clone();
                    self.session = Some(session);
                    self.scroll = SCROLL_BOTTOM;
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to load session: {}", e),
                    ));
                }
            }
            return;
        }

        // Check for /model command - open model picker
        if message.trim() == "/model" || message.trim().starts_with("/model ") {
            let direct_model = message
                .trim()
                .strip_prefix("/model")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());

            if let Some(model_str) = direct_model {
                // Direct set: /model provider/model-name
                self.active_model = Some(model_str.to_string());
                if let Some(session) = self.session.as_mut() {
                    session.metadata.model = Some(model_str.to_string());
                }
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Model set to: {}", model_str),
                ));
            } else {
                // Open model picker
                self.open_model_picker(config).await;
            }
            return;
        }

        // Check for /new command to start a fresh session
        if message.trim() == "/new" {
            self.session = None;
            self.messages.clear();
            self.messages.push(ChatMessage::new(
                "system",
                "Started a new session. Previous session was saved.",
            ));
            return;
        }

        // Add user message
        self.messages
            .push(ChatMessage::new("user", message.clone()));

        // Auto-scroll to bottom when user sends a message
        self.scroll = SCROLL_BOTTOM;

        let current_agent = self.current_agent.clone();
        let model = self
            .active_model
            .clone()
            .or_else(|| {
                config
                    .agents
                    .get(&current_agent)
                    .and_then(|agent| agent.model.clone())
            })
            .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
            .or_else(|| config.default_model.clone());

        // Initialize session if needed
        if self.session.is_none() {
            match Session::new().await {
                Ok(session) => {
                    self.session = Some(session);
                }
                Err(err) => {
                    tracing::error!(error = %err, "Failed to create session");
                    self.messages
                        .push(ChatMessage::new("assistant", format!("Error: {err}")));
                    return;
                }
            }
        }

        let session = match self.session.as_mut() {
            Some(session) => session,
            None => {
                self.messages.push(ChatMessage::new(
                    "assistant",
                    "Error: session not initialized",
                ));
                return;
            }
        };

        if let Some(model) = model {
            session.metadata.model = Some(model);
        }

        session.agent = current_agent;

        // Set processing state
        self.is_processing = true;
        self.processing_message = Some("Thinking...".to_string());
        self.current_tool = None;

        // Create channel for async communication
        let (tx, rx) = mpsc::channel(100);
        self.response_rx = Some(rx);

        // Clone session for async processing
        let session_clone = session.clone();
        let message_clone = message.clone();

        // Spawn async task to process the message with event streaming
        tokio::spawn(async move {
            let mut session = session_clone;
            if let Err(err) = session.prompt_with_events(&message_clone, tx.clone()).await {
                tracing::error!(error = %err, "Agent processing failed");
                let _ = tx.send(SessionEvent::Error(format!("Error: {err}"))).await;
                let _ = tx.send(SessionEvent::Done).await;
            }
        });
    }

    fn handle_response(&mut self, event: SessionEvent) {
        // Auto-scroll to bottom when new content arrives
        self.scroll = SCROLL_BOTTOM;

        match event {
            SessionEvent::Thinking => {
                self.processing_message = Some("Thinking...".to_string());
                self.current_tool = None;
            }
            SessionEvent::ToolCallStart { name, arguments } => {
                self.processing_message = Some(format!("Running {}...", name));
                self.current_tool = Some(name.clone());
                self.messages.push(
                    ChatMessage::new("tool", format!("🔧 {}", name))
                        .with_message_type(MessageType::ToolCall { name, arguments }),
                );
            }
            SessionEvent::ToolCallComplete {
                name,
                output,
                success,
            } => {
                let icon = if success { "✓" } else { "✗" };
                self.messages.push(
                    ChatMessage::new("tool", format!("{} {}", icon, name))
                        .with_message_type(MessageType::ToolResult { name, output }),
                );
                self.current_tool = None;
                self.processing_message = Some("Thinking...".to_string());
            }
            SessionEvent::TextChunk(_text) => {
                // Could be used for streaming text display in the future
            }
            SessionEvent::TextComplete(text) => {
                if !text.is_empty() {
                    self.messages.push(ChatMessage::new("assistant", text));
                }
            }
            SessionEvent::Error(err) => {
                self.messages
                    .push(ChatMessage::new("assistant", format!("Error: {}", err)));
            }
            SessionEvent::Done => {
                self.is_processing = false;
                self.processing_message = None;
                self.current_tool = None;
                self.response_rx = None;
            }
        }
    }

    /// Handle a swarm event
    fn handle_swarm_event(&mut self, event: SwarmEvent) {
        self.swarm_state.handle_event(event.clone());

        // When swarm completes, switch back to chat view with summary
        if let SwarmEvent::Complete { success, ref stats } = event {
            self.view_mode = ViewMode::Chat;
            let summary = if success {
                format!(
                    "Swarm completed successfully.\n\
                     Subtasks: {} completed, {} failed\n\
                     Total tool calls: {}\n\
                     Time: {:.1}s (speedup: {:.1}x)",
                    stats.subagents_completed,
                    stats.subagents_failed,
                    stats.total_tool_calls,
                    stats.execution_time_ms as f64 / 1000.0,
                    stats.speedup_factor
                )
            } else {
                format!(
                    "Swarm completed with failures.\n\
                     Subtasks: {} completed, {} failed\n\
                     Check the subtask results for details.",
                    stats.subagents_completed, stats.subagents_failed
                )
            };
            self.messages.push(ChatMessage::new("system", &summary));
            self.swarm_rx = None;
        }

        if let SwarmEvent::Error(ref err) = event {
            self.messages
                .push(ChatMessage::new("system", &format!("Swarm error: {}", err)));
        }
    }

    /// Start swarm execution for a task
    async fn start_swarm_execution(&mut self, task: String, config: &Config) {
        // Add user message
        self.messages
            .push(ChatMessage::new("user", format!("/swarm {}", task)));

        // Get model from config
        let model = config
            .default_model
            .clone()
            .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok());

        // Configure swarm
        let swarm_config = SwarmConfig {
            model,
            max_subagents: 10,
            max_steps_per_subagent: 50,
            worktree_enabled: true,
            worktree_auto_merge: true,
            working_dir: Some(
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            ),
            ..Default::default()
        };

        // Create channel for swarm events
        let (tx, rx) = mpsc::channel(100);
        self.swarm_rx = Some(rx);

        // Switch to swarm view
        self.view_mode = ViewMode::Swarm;
        self.swarm_state = SwarmViewState::new();

        // Send initial event
        let _ = tx
            .send(SwarmEvent::Started {
                task: task.clone(),
                total_subtasks: 0,
            })
            .await;

        // Spawn swarm execution — executor emits all events via event_tx
        let task_clone = task;
        tokio::spawn(async move {
            // Create executor with event channel — it handles decomposition + execution
            let executor = SwarmExecutor::new(swarm_config).with_event_tx(tx.clone());
            let result = executor
                .execute(&task_clone, DecompositionStrategy::Automatic)
                .await;

            match result {
                Ok(swarm_result) => {
                    let _ = tx
                        .send(SwarmEvent::Complete {
                            success: swarm_result.success,
                            stats: swarm_result.stats,
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx.send(SwarmEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Populate and open the model picker overlay
    async fn open_model_picker(&mut self, config: &Config) {
        let mut models: Vec<(String, String)> = Vec::new();

        // Try to build provider registry and list models
        match crate::provider::ProviderRegistry::from_vault().await {
            Ok(registry) => {
                for provider_name in registry.list() {
                    if let Some(provider) = registry.get(provider_name) {
                        match provider.list_models().await {
                            Ok(model_list) => {
                                for m in model_list {
                                    let label = format!("{}/{}", provider_name, m.id);
                                    let value = format!("{}/{}", provider_name, m.id);
                                    models.push((label, value));
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Failed to list models for {}: {}", provider_name, e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to load provider registry: {}", e);
            }
        }

        // Fallback: also try from config
        if models.is_empty() {
            if let Ok(registry) = crate::provider::ProviderRegistry::from_config(config).await {
                for provider_name in registry.list() {
                    if let Some(provider) = registry.get(provider_name) {
                        if let Ok(model_list) = provider.list_models().await {
                            for m in model_list {
                                let label = format!("{}/{}", provider_name, m.id);
                                let value = format!("{}/{}", provider_name, m.id);
                                models.push((label, value));
                            }
                        }
                    }
                }
            }
        }

        if models.is_empty() {
            self.messages.push(ChatMessage::new(
                "system",
                "No models found. Check provider configuration (Vault or config).",
            ));
        } else {
            // Sort models by provider then name
            models.sort_by(|a, b| a.0.cmp(&b.0));
            self.model_picker_list = models;
            self.model_picker_selected = 0;
            self.model_picker_filter.clear();
            self.view_mode = ViewMode::ModelPicker;
        }
    }

    /// Get filtered model list
    fn filtered_models(&self) -> Vec<(usize, &(String, String))> {
        if self.model_picker_filter.is_empty() {
            self.model_picker_list.iter().enumerate().collect()
        } else {
            let filter = self.model_picker_filter.to_lowercase();
            self.model_picker_list
                .iter()
                .enumerate()
                .filter(|(_, (label, _))| label.to_lowercase().contains(&filter))
                .collect()
        }
    }

    fn navigate_history(&mut self, direction: isize) {
        if self.command_history.is_empty() {
            return;
        }

        let history_len = self.command_history.len();
        let new_index = match self.history_index {
            Some(current) => {
                let new = current as isize + direction;
                if new < 0 {
                    None
                } else if new >= history_len as isize {
                    Some(history_len - 1)
                } else {
                    Some(new as usize)
                }
            }
            None => {
                if direction > 0 {
                    Some(0)
                } else {
                    Some(history_len.saturating_sub(1))
                }
            }
        };

        self.history_index = new_index;
        if let Some(index) = new_index {
            self.input = self.command_history[index].clone();
            self.cursor_position = self.input.len();
        } else {
            self.input.clear();
            self.cursor_position = 0;
        }
    }

    fn search_history(&mut self) {
        // Enhanced search: find commands matching current input prefix
        if self.command_history.is_empty() {
            return;
        }

        let search_term = self.input.trim().to_lowercase();

        if search_term.is_empty() {
            // Empty search - show most recent
            if !self.command_history.is_empty() {
                self.input = self.command_history.last().unwrap().clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(self.command_history.len() - 1);
            }
            return;
        }

        // Find the most recent command that starts with the search term
        for (index, cmd) in self.command_history.iter().enumerate().rev() {
            if cmd.to_lowercase().starts_with(&search_term) {
                self.input = cmd.clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(index);
                return;
            }
        }

        // If no prefix match, search for contains
        for (index, cmd) in self.command_history.iter().enumerate().rev() {
            if cmd.to_lowercase().contains(&search_term) {
                self.input = cmd.clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(index);
                return;
            }
        }
    }
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    // Load configuration and theme
    let mut config = Config::load().await?;
    let mut theme = crate::tui::theme_utils::validate_theme(&config.load_theme());

    // Track last config modification time for hot-reloading
    let _config_paths = vec![
        std::path::PathBuf::from("./codetether.toml"),
        std::path::PathBuf::from("./.codetether/config.toml"),
    ];

    let _global_config_path = directories::ProjectDirs::from("com", "codetether", "codetether")
        .map(|dirs| dirs.config_dir().join("config.toml"));

    let mut last_check = std::time::Instant::now();

    loop {
        // Check for theme changes if hot-reload is enabled
        if config.ui.hot_reload && last_check.elapsed() > std::time::Duration::from_secs(2) {
            if let Ok(new_config) = Config::load().await {
                if new_config.ui.theme != config.ui.theme
                    || new_config.ui.custom_theme != config.ui.custom_theme
                {
                    theme = crate::tui::theme_utils::validate_theme(&new_config.load_theme());
                    config = new_config;
                }
            }
            last_check = std::time::Instant::now();
        }

        terminal.draw(|f| ui(f, &mut app, &theme))?;

        // Update max_scroll estimate for scroll key handlers
        // This needs to roughly match what ui() calculates
        let terminal_height = terminal.size()?.height.saturating_sub(6) as usize;
        let estimated_lines = app.messages.len() * 4; // rough estimate
        app.last_max_scroll = estimated_lines.saturating_sub(terminal_height);

        // Drain all pending async responses
        if let Some(mut rx) = app.response_rx.take() {
            while let Ok(response) = rx.try_recv() {
                app.handle_response(response);
            }
            app.response_rx = Some(rx);
        }

        // Drain all pending swarm events
        if let Some(mut rx) = app.swarm_rx.take() {
            while let Ok(event) = rx.try_recv() {
                app.handle_swarm_event(event);
            }
            app.swarm_rx = Some(rx);
        }

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Help overlay
                if app.show_help {
                    if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
                        app.show_help = false;
                    }
                    continue;
                }

                // Model picker overlay
                if app.view_mode == ViewMode::ModelPicker {
                    match key.code {
                        KeyCode::Esc => {
                            app.view_mode = ViewMode::Chat;
                        }
                        KeyCode::Up | KeyCode::Char('k') if !key.modifiers.contains(KeyModifiers::ALT) => {
                            if app.model_picker_selected > 0 {
                                app.model_picker_selected -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') if !key.modifiers.contains(KeyModifiers::ALT) => {
                            let filtered = app.filtered_models();
                            if app.model_picker_selected < filtered.len().saturating_sub(1) {
                                app.model_picker_selected += 1;
                            }
                        }
                        KeyCode::Enter => {
                            let filtered = app.filtered_models();
                            if let Some((_, (label, value))) = filtered.get(app.model_picker_selected) {
                                let label = label.clone();
                                let value = value.clone();
                                app.active_model = Some(value.clone());
                                if let Some(session) = app.session.as_mut() {
                                    session.metadata.model = Some(value.clone());
                                }
                                app.messages.push(ChatMessage::new(
                                    "system",
                                    format!("Model set to: {}", label),
                                ));
                                app.view_mode = ViewMode::Chat;
                            }
                        }
                        KeyCode::Backspace => {
                            app.model_picker_filter.pop();
                            app.model_picker_selected = 0;
                        }
                        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) && !key.modifiers.contains(KeyModifiers::ALT) => {
                            app.model_picker_filter.push(c);
                            app.model_picker_selected = 0;
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        _ => {}
                    }
                    continue;
                }

                // Session picker overlay - handle specially
                if app.view_mode == ViewMode::SessionPicker {
                    match key.code {
                        KeyCode::Esc => {
                            app.view_mode = ViewMode::Chat;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.session_picker_selected > 0 {
                                app.session_picker_selected -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.session_picker_selected
                                < app.session_picker_list.len().saturating_sub(1)
                            {
                                app.session_picker_selected += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(session_summary) =
                                app.session_picker_list.get(app.session_picker_selected)
                            {
                                let session_id = session_summary.id.clone();
                                match Session::load(&session_id).await {
                                    Ok(session) => {
                                        app.messages.clear();
                                        app.messages.push(ChatMessage::new("system", format!(
                                            "Resumed session: {}\nCreated: {}\n{} messages loaded",
                                            session.title.as_deref().unwrap_or("(untitled)"),
                                            session.created_at.format("%Y-%m-%d %H:%M"),
                                            session.messages.len()
                                        )));

                                        for msg in &session.messages {
                                            let role_str = match msg.role {
                                                Role::System => "system",
                                                Role::User => "user",
                                                Role::Assistant => "assistant",
                                                Role::Tool => "tool",
                                            };

                                            let text = msg
                                                .content
                                                .iter()
                                                .filter_map(|part| {
                                                    if let ContentPart::Text { text } = part {
                                                        Some(text.as_str())
                                                    } else {
                                                        None
                                                    }
                                                })
                                                .collect::<Vec<_>>()
                                                .join("\n");

                                            if !text.is_empty() {
                                                app.messages.push(ChatMessage::new(role_str, text));
                                            }
                                        }

                                        app.current_agent = session.agent.clone();
                                        app.session = Some(session);
                                        app.scroll = SCROLL_BOTTOM;
                                        app.view_mode = ViewMode::Chat;
                                    }
                                    Err(e) => {
                                        app.messages.push(ChatMessage::new(
                                            "system",
                                            format!("Failed to load session: {}", e),
                                        ));
                                        app.view_mode = ViewMode::Chat;
                                    }
                                }
                            }
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        _ => {}
                    }
                    continue;
                }

                // Swarm view key handling
                if app.view_mode == ViewMode::Swarm {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Esc => {
                            if app.swarm_state.detail_mode {
                                app.swarm_state.exit_detail();
                            } else {
                                app.view_mode = ViewMode::Chat;
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.swarm_state.detail_mode {
                                // In detail mode, Up/Down switch between agents
                                app.swarm_state.exit_detail();
                                app.swarm_state.select_prev();
                                app.swarm_state.enter_detail();
                            } else {
                                app.swarm_state.select_prev();
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.swarm_state.detail_mode {
                                app.swarm_state.exit_detail();
                                app.swarm_state.select_next();
                                app.swarm_state.enter_detail();
                            } else {
                                app.swarm_state.select_next();
                            }
                        }
                        KeyCode::Enter => {
                            if !app.swarm_state.detail_mode {
                                app.swarm_state.enter_detail();
                            }
                        }
                        KeyCode::PageDown => {
                            app.swarm_state.detail_scroll_down(10);
                        }
                        KeyCode::PageUp => {
                            app.swarm_state.detail_scroll_up(10);
                        }
                        KeyCode::Char('?') => {
                            app.show_help = true;
                        }
                        KeyCode::F(2) => {
                            app.view_mode = ViewMode::Chat;
                        }
                        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.view_mode = ViewMode::Chat;
                        }
                        _ => {}
                    }
                    continue;
                }

                match key.code {
                    // Quit
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }

                    // Help
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }

                    // Toggle view mode (F2 or Ctrl+S)
                    KeyCode::F(2) => {
                        app.view_mode = match app.view_mode {
                            ViewMode::Chat | ViewMode::SessionPicker | ViewMode::ModelPicker => ViewMode::Swarm,
                            ViewMode::Swarm => ViewMode::Chat,
                        };
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.view_mode = match app.view_mode {
                            ViewMode::Chat | ViewMode::SessionPicker | ViewMode::ModelPicker => ViewMode::Swarm,
                            ViewMode::Swarm => ViewMode::Chat,
                        };
                    }

                    // Escape - return to chat from swarm/picker view
                    KeyCode::Esc => {
                        if app.view_mode == ViewMode::Swarm
                            || app.view_mode == ViewMode::SessionPicker
                            || app.view_mode == ViewMode::ModelPicker
                        {
                            app.view_mode = ViewMode::Chat;
                        }
                    }

                    // Model picker (Ctrl+M)
                    KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.open_model_picker(&config).await;
                    }

                    // Switch agent
                    KeyCode::Tab => {
                        app.current_agent = if app.current_agent == "build" {
                            "plan".to_string()
                        } else {
                            "build".to_string()
                        };
                    }

                    // Submit message
                    KeyCode::Enter => {
                        app.submit_message(&config).await;
                    }

                    // Vim-style scrolling (Alt + j/k)
                    KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::ALT) => {
                        if app.scroll < SCROLL_BOTTOM {
                            app.scroll = app.scroll.saturating_add(1);
                        }
                    }
                    KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::ALT) => {
                        if app.scroll >= SCROLL_BOTTOM {
                            app.scroll = app.last_max_scroll; // Leave auto-scroll mode
                        }
                        app.scroll = app.scroll.saturating_sub(1);
                    }

                    // Command history
                    KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.search_history();
                    }
                    KeyCode::Up if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.navigate_history(-1);
                    }
                    KeyCode::Down if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.navigate_history(1);
                    }

                    // Additional Vim-style navigation (with modifiers to avoid conflicts)
                    KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.scroll = 0; // Go to top
                    }
                    KeyCode::Char('G') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        // Go to bottom (auto-scroll)
                        app.scroll = SCROLL_BOTTOM;
                    }

                    // Enhanced scrolling (with Alt to avoid conflicts)
                    KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::ALT) => {
                        // Half page down
                        if app.scroll < SCROLL_BOTTOM {
                            app.scroll = app.scroll.saturating_add(5);
                        }
                    }
                    KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::ALT) => {
                        // Half page up
                        if app.scroll >= SCROLL_BOTTOM {
                            app.scroll = app.last_max_scroll;
                        }
                        app.scroll = app.scroll.saturating_sub(5);
                    }

                    // Text input
                    KeyCode::Char(c) => {
                        app.input.insert(app.cursor_position, c);
                        app.cursor_position += 1;
                    }
                    KeyCode::Backspace => {
                        if app.cursor_position > 0 {
                            app.cursor_position -= 1;
                            app.input.remove(app.cursor_position);
                        }
                    }
                    KeyCode::Delete => {
                        if app.cursor_position < app.input.len() {
                            app.input.remove(app.cursor_position);
                        }
                    }
                    KeyCode::Left => {
                        app.cursor_position = app.cursor_position.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        if app.cursor_position < app.input.len() {
                            app.cursor_position += 1;
                        }
                    }
                    KeyCode::Home => {
                        app.cursor_position = 0;
                    }
                    KeyCode::End => {
                        app.cursor_position = app.input.len();
                    }

                    // Scroll (normalize first to handle SCROLL_BOTTOM sentinel)
                    KeyCode::Up => {
                        if app.scroll >= SCROLL_BOTTOM {
                            app.scroll = app.last_max_scroll; // Leave auto-scroll mode
                        }
                        app.scroll = app.scroll.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        if app.scroll < SCROLL_BOTTOM {
                            app.scroll = app.scroll.saturating_add(1);
                        }
                    }
                    KeyCode::PageUp => {
                        if app.scroll >= SCROLL_BOTTOM {
                            app.scroll = app.last_max_scroll;
                        }
                        app.scroll = app.scroll.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        if app.scroll < SCROLL_BOTTOM {
                            app.scroll = app.scroll.saturating_add(10);
                        }
                    }

                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App, theme: &Theme) {
    // Check view mode
    if app.view_mode == ViewMode::Swarm {
        // Render swarm view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Swarm view
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        // Swarm view
        render_swarm_view(f, &mut app.swarm_state, chunks[0]);

        // Input area (for returning to chat)
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc, Ctrl+S, or /view to return to chat ")
            .border_style(Style::default().fg(Color::Cyan));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        // Status bar
        let status_line = if app.swarm_state.detail_mode {
            Line::from(vec![
                Span::styled(
                    " AGENT DETAIL ",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                Span::raw(" | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back to list | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Prev/Next agent | "),
                Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
                Span::raw(": Scroll"),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    " SWARM MODE ",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                Span::raw(" | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Select | "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": Detail | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back | "),
                Span::styled("Ctrl+S", Style::default().fg(Color::Yellow)),
                Span::raw(": Toggle view"),
            ])
        };
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Model picker view
    if app.view_mode == ViewMode::ModelPicker {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);

        let filter_display = if app.model_picker_filter.is_empty() {
            "type to filter".to_string()
        } else {
            format!("filter: {}", app.model_picker_filter)
        };

        let picker_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Select Model (↑↓ navigate, Enter select, Esc cancel) [{}] ", filter_display))
            .border_style(Style::default().fg(Color::Magenta));

        let filtered = app.filtered_models();
        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        if let Some(ref active) = app.active_model {
            list_lines.push(Line::styled(
                format!("  Current: {}", active),
                Style::default().fg(Color::Green).add_modifier(Modifier::DIM),
            ));
            list_lines.push(Line::from(""));
        }

        if filtered.is_empty() {
            list_lines.push(Line::styled(
                "  No models match filter",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            let mut current_provider = String::new();
            for (display_idx, (_, (label, _))) in filtered.iter().enumerate() {
                let provider = label.split('/').next().unwrap_or("");
                if provider != current_provider {
                    if !current_provider.is_empty() {
                        list_lines.push(Line::from(""));
                    }
                    list_lines.push(Line::styled(
                        format!("  ─── {} ───", provider),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ));
                    current_provider = provider.to_string();
                }

                let is_selected = display_idx == app.model_picker_selected;
                let is_active = app.active_model.as_deref() == Some(label.as_str());
                let marker = if is_selected { "▶" } else { " " };
                let active_marker = if is_active { " ✓" } else { "" };
                let model_name = label.split('/').skip(1).collect::<Vec<_>>().join("/");

                let style = if is_selected {
                    Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)
                } else if is_active {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                list_lines.push(Line::styled(
                    format!("  {} {}{}", marker, model_name, active_marker),
                    style,
                ));
            }
        }

        let list = Paragraph::new(list_lines)
            .block(picker_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, area);
        return;
    }

    // Session picker view
    if app.view_mode == ViewMode::SessionPicker {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Session list
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        // Session list
        let list_block = Block::default()
            .borders(Borders::ALL)
            .title(" Select Session (↑↓ to navigate, Enter to load, Esc to cancel) ")
            .border_style(Style::default().fg(Color::Cyan));

        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        for (i, session) in app.session_picker_list.iter().enumerate() {
            let is_selected = i == app.session_picker_selected;
            let title = session.title.as_deref().unwrap_or("(untitled)");
            let date = session.updated_at.format("%Y-%m-%d %H:%M");
            let line_str = format!(
                " {} {} - {} ({} msgs)",
                if is_selected { "▶" } else { " " },
                title,
                date,
                session.message_count
            );

            let style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            list_lines.push(Line::styled(line_str, style));

            // Add agent info on next line for selected item
            if is_selected {
                list_lines.push(Line::styled(
                    format!("   Agent: {} | ID: {}", session.agent, session.id),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }

        let list = Paragraph::new(list_lines)
            .block(list_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, chunks[0]);

        // Status bar
        let status = Paragraph::new(Line::from(vec![
            Span::styled(
                " SESSION PICKER ",
                Style::default().fg(Color::Black).bg(Color::Cyan),
            ),
            Span::raw(" | "),
            Span::styled("↑↓/jk", Style::default().fg(Color::Yellow)),
            Span::raw(": Navigate | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": Load | "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Cancel"),
        ]));
        f.render_widget(status, chunks[1]);
        return;
    }

    // Chat view (default)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Messages
            Constraint::Length(3), // Input
            Constraint::Length(1), // Status bar
        ])
        .split(f.area());

    // Messages area with theme-based styling
    let messages_area = chunks[0];
    let model_label = app.active_model.as_deref().unwrap_or("auto");
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" CodeTether Agent [{}] model:{} ", app.current_agent, model_label))
        .border_style(Style::default().fg(theme.border_color.to_color()));

    // Create scrollable message content
    let mut message_lines = Vec::new();
    let max_width = messages_area.width.saturating_sub(4) as usize;

    for message in &app.messages {
        // Message header with theme-based styling
        let role_style = theme.get_role_style(&message.role);

        let header_line = Line::from(vec![
            Span::styled(
                format!("[{}] ", message.timestamp),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled(&message.role, role_style),
        ]);
        message_lines.push(header_line);

        // Format message content based on message type
        match &message.message_type {
            MessageType::ToolCall { name, arguments } => {
                // Tool call display with distinct styling
                let tool_header = Line::from(vec![
                    Span::styled("  🔧 ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("Tool: {}", name),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);
                message_lines.push(tool_header);

                // Arguments (truncated if too long)
                let args_str = if arguments.len() > 200 {
                    format!("{}...", &arguments[..197])
                } else {
                    arguments.clone()
                };
                let args_line = Line::from(vec![
                    Span::styled("     ", Style::default()),
                    Span::styled(args_str, Style::default().fg(Color::DarkGray)),
                ]);
                message_lines.push(args_line);
            }
            MessageType::ToolResult { name, output } => {
                // Tool result display
                let result_header = Line::from(vec![
                    Span::styled("  ✅ ", Style::default().fg(Color::Green)),
                    Span::styled(
                        format!("Result from {}", name),
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);
                message_lines.push(result_header);

                // Output (truncated if too long)
                let output_str = if output.len() > 300 {
                    format!("{}... (truncated)", &output[..297])
                } else {
                    output.clone()
                };
                let output_lines: Vec<&str> = output_str.lines().collect();
                for line in output_lines.iter().take(5) {
                    let output_line = Line::from(vec![
                        Span::styled("     ", Style::default()),
                        Span::styled(line.to_string(), Style::default().fg(Color::DarkGray)),
                    ]);
                    message_lines.push(output_line);
                }
                if output_lines.len() > 5 {
                    message_lines.push(Line::from(vec![
                        Span::styled("     ", Style::default()),
                        Span::styled(
                            format!("... and {} more lines", output_lines.len() - 5),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]));
                }
            }
            MessageType::Text(text) => {
                // Regular text message - use the stored text content
                let formatter = MessageFormatter::new(max_width);
                let formatted_content = formatter.format_content(text, &message.role);
                message_lines.extend(formatted_content);
            }
        }

        // Add spacing between messages
        message_lines.push(Line::from(""));
    }

    // Show processing indicator if active
    if app.is_processing {
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let spinner_idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
            / 100) as usize
            % spinner.len();

        let processing_line = Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("assistant", theme.get_role_style("assistant")),
        ]);
        message_lines.push(processing_line);

        // Show different colors based on state
        let (status_text, status_color) = if let Some(ref tool) = app.current_tool {
            (
                format!("  {} Running: {}", spinner[spinner_idx], tool),
                Color::Cyan,
            )
        } else {
            (
                format!(
                    "  {} {}",
                    spinner[spinner_idx],
                    app.processing_message.as_deref().unwrap_or("Thinking...")
                ),
                Color::Yellow,
            )
        };

        let indicator_line = Line::from(vec![Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        )]);
        message_lines.push(indicator_line);
        message_lines.push(Line::from(""));
    }

    // Calculate scroll position
    let total_lines = message_lines.len();
    let visible_lines = messages_area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_lines);
    // SCROLL_BOTTOM means "stick to bottom", otherwise clamp to max_scroll
    let scroll = if app.scroll >= SCROLL_BOTTOM {
        max_scroll
    } else {
        app.scroll.min(max_scroll)
    };

    // Render messages with scrolling
    let messages_paragraph = Paragraph::new(
        message_lines[scroll..(scroll + visible_lines.min(total_lines)).min(total_lines)].to_vec(),
    )
    .block(messages_block.clone())
    .wrap(Wrap { trim: false });

    f.render_widget(messages_paragraph, messages_area);

    // Render scrollbar if needed
    if total_lines > visible_lines {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .symbols(ratatui::symbols::scrollbar::VERTICAL)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(total_lines).position(scroll);

        let scrollbar_area = Rect::new(
            messages_area.right() - 1,
            messages_area.top() + 1,
            1,
            messages_area.height - 2,
        );

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    // Input area
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(if app.is_processing {
            " Message (Processing...) "
        } else {
            " Message (Enter to send) "
        })
        .border_style(Style::default().fg(if app.is_processing {
            Color::Yellow
        } else {
            theme.input_border_color.to_color()
        }));

    let input = Paragraph::new(app.input.as_str())
        .block(input_block)
        .wrap(Wrap { trim: false });
    f.render_widget(input, chunks[1]);

    // Cursor
    f.set_cursor_position((
        chunks[1].x + app.cursor_position as u16 + 1,
        chunks[1].y + 1,
    ));

    // Enhanced status bar with token display
    let token_display = TokenDisplay::new();
    let status = Paragraph::new(token_display.create_status_bar(theme));
    f.render_widget(status, chunks[2]);

    // Help overlay
    if app.show_help {
        let area = centered_rect(60, 60, f.area());
        f.render_widget(Clear, area);

        // Enhanced token usage details
        let token_display = TokenDisplay::new();
        let token_info = token_display.create_detailed_display();

        let help_text: Vec<String> = vec![
            "".to_string(),
            "  KEYBOARD SHORTCUTS".to_string(),
            "  ==================".to_string(),
            "".to_string(),
            "  Enter        Send message".to_string(),
            "  Tab          Switch between build/plan agents".to_string(),
            "  Ctrl+S       Toggle swarm view".to_string(),
            "  Ctrl+M       Open model picker".to_string(),
            "  Ctrl+C       Quit".to_string(),
            "  ?            Toggle this help".to_string(),
            "".to_string(),
            "  SLASH COMMANDS".to_string(),
            "  /swarm <task>   Run task in parallel swarm mode".to_string(),
            "  /sessions       Open session picker to resume".to_string(),
            "  /resume         Resume most recent session".to_string(),
            "  /resume <id>    Resume specific session by ID".to_string(),
            "  /new            Start a fresh session".to_string(),
            "  /model          Open model picker (or /model <name>)".to_string(),
            "  /view           Toggle swarm view".to_string(),
            "".to_string(),
            "  VIM-STYLE NAVIGATION".to_string(),
            "  Alt+j        Scroll down".to_string(),
            "  Alt+k        Scroll up".to_string(),
            "  Ctrl+g       Go to top".to_string(),
            "  Ctrl+G       Go to bottom".to_string(),
            "".to_string(),
            "  SCROLLING".to_string(),
            "  Up/Down      Scroll messages".to_string(),
            "  PageUp/Dn    Scroll one page".to_string(),
            "  Alt+u/d      Scroll half page".to_string(),
            "".to_string(),
            "  COMMAND HISTORY".to_string(),
            "  Ctrl+R       Search history".to_string(),
            "  Ctrl+Up/Dn   Navigate history".to_string(),
            "".to_string(),
            "  Press ? or Esc to close".to_string(),
            "".to_string(),
        ];

        let mut combined_text = token_info;
        combined_text.extend(help_text);

        let help = Paragraph::new(combined_text.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Help ")
                    .border_style(Style::default().fg(theme.help_border_color.to_color())),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(help, area);
    }
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
