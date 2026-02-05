//! Terminal User Interface
//!
//! Interactive TUI using Ratatui

pub mod message_formatter;
pub mod theme;
pub mod theme_utils;
pub mod token_display;

use crate::config::Config;
use crate::session::Session;
use crate::tui::message_formatter::MessageFormatter;
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
    response_rx: Option<mpsc::Receiver<AgentResponse>>,
}

struct ChatMessage {
    role: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

/// Response from the agent
#[derive(Debug, Clone)]
enum AgentResponse {
    Text(String),
    ToolCall { name: String, arguments: String },
    ToolResult { name: String, output: String },
    Error(String),
    Complete,
}

impl ChatMessage {
    fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: content.into(),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            message_type: MessageType::Text(String::new()),
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
                ChatMessage::new("system", "Welcome to CodeTether Agent! Type a message to get started, or press ? for help."),
                ChatMessage::new("assistant", "Enhanced message display is now active! Features include:\n\n• Proper text wrapping at terminal width\n• Scrollbar indicating conversation position\n• Distinct colors for different message types\n• Code block formatting with syntax highlighting\n\nExample code block:\n```rust\nfn main() {\n    println!(\"Hello, World!\");\n}\n```\n\nTry scrolling with ↑/↓ keys and PageUp/PageDown!"),
            ],
            current_agent: "build".to_string(),
            scroll: 0,
            show_help: false,
            command_history: Vec::new(),
            history_index: None,
            session: None,
            is_processing: false,
            processing_message: None,
            response_rx: None,
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

        // Add user message
        self.messages.push(ChatMessage::new("user", message.clone()));

        let current_agent = self.current_agent.clone();
        let model = config
            .agents
            .get(&current_agent)
            .and_then(|agent| agent.model.clone())
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
                    self.messages.push(ChatMessage::new("assistant", format!("Error: {err}")));
                    return;
                }
            }
        }

        let session = match self.session.as_mut() {
            Some(session) => session,
            None => {
                self.messages.push(ChatMessage::new("assistant", "Error: session not initialized"));
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

        // Create channel for async communication
        let (tx, rx) = mpsc::channel(100);
        self.response_rx = Some(rx);

        // Clone session for async processing
        let session_clone = session.clone();
        let message_clone = message.clone();

        // Spawn async task to process the message
        tokio::spawn(async move {
            let mut session = session_clone;
            match session.prompt(&message_clone).await {
                Ok(result) => {
                    let _ = tx.send(AgentResponse::Text(result.text)).await;
                    let _ = tx.send(AgentResponse::Complete).await;
                }
                Err(err) => {
                    tracing::error!(error = %err, "Agent processing failed");
                    let _ = tx.send(AgentResponse::Error(format!("Error: {err}"))).await;
                    let _ = tx.send(AgentResponse::Complete).await;
                }
            }
        });
    }

    fn handle_response(&mut self, response: AgentResponse) {
        match response {
            AgentResponse::Text(text) => {
                self.messages.push(ChatMessage::new("assistant", text));
            }
            AgentResponse::ToolCall { name, arguments } => {
                self.messages.push(
                    ChatMessage::new("tool", format!("Calling tool: {name}"))
                        .with_message_type(MessageType::ToolCall { name, arguments }),
                );
            }
            AgentResponse::ToolResult { name, output } => {
                self.messages.push(
                    ChatMessage::new("tool", format!("Tool result: {name}"))
                        .with_message_type(MessageType::ToolResult { name, output }),
                );
            }
            AgentResponse::Error(err) => {
                self.messages.push(ChatMessage::new("assistant", err));
            }
            AgentResponse::Complete => {
                self.is_processing = false;
                self.processing_message = None;
                self.response_rx = None;
            }
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

        terminal.draw(|f| ui(f, &app, &theme))?;

        // Check for async responses
        if let Some(ref mut rx) = app.response_rx {
            if let Ok(response) = rx.try_recv() {
                app.handle_response(response);
            }
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

                    // Vim-style navigation
                    KeyCode::Char('h') if !app.show_help => {
                        app.cursor_position = app.cursor_position.saturating_sub(1);
                    }
                    KeyCode::Char('j') if !app.show_help => {
                        app.scroll = app.scroll.saturating_add(1);
                    }
                    KeyCode::Char('k') if !app.show_help => {
                        app.scroll = app.scroll.saturating_sub(1);
                    }
                    KeyCode::Char('l') if !app.show_help => {
                        if app.cursor_position < app.input.len() {
                            app.cursor_position += 1;
                        }
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

                    // Additional Vim-style navigation
                    KeyCode::Char('g') if key.modifiers.is_empty() => {
                        app.scroll = 0; // Go to top
                    }
                    KeyCode::Char('G') if key.modifiers.is_empty() => {
                        // Go to bottom - will be calculated properly in the draw function
                        app.scroll = usize::MAX; // Use max value, will be clamped
                    }

                    // Enhanced scrolling
                    KeyCode::Char('d') if key.modifiers.is_empty() => {
                        // Vim-style page down (half page)
                        app.scroll = app.scroll.saturating_add(5);
                    }
                    KeyCode::Char('u') if key.modifiers.is_empty() => {
                        // Vim-style page up (half page)
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

                    // Scroll
                    KeyCode::Up => {
                        app.scroll = app.scroll.saturating_sub(1);
                    }
                    KeyCode::Down => {
                        app.scroll = app.scroll.saturating_add(1);
                    }
                    KeyCode::PageUp => {
                        app.scroll = app.scroll.saturating_sub(10);
                    }
                    KeyCode::PageDown => {
                        app.scroll = app.scroll.saturating_add(10);
                    }

                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App, theme: &Theme) {
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
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" CodeTether Agent [{}] ", app.current_agent))
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
                    Span::styled(format!("Tool: {}", name), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
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
                    Span::styled(format!("Result from {}", name), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
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
                        Span::styled(format!("... and {} more lines", output_lines.len() - 5), Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)),
                    ]));
                }
            }
            _ => {
                // Regular text message
                let formatter = MessageFormatter::new(max_width);
                let formatted_content = formatter.format_content(&message.content, &message.role);
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
            .as_millis() / 100) as usize % spinner.len();
        
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
        
        let indicator_line = Line::from(vec![
            Span::styled(
                format!("  {} {}", spinner[spinner_idx], app.processing_message.as_deref().unwrap_or("Thinking...")),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]);
        message_lines.push(indicator_line);
        message_lines.push(Line::from(""));
    }

    // Calculate scroll position
    let total_lines = message_lines.len();
    let visible_lines = messages_area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_lines);
    let scroll = app.scroll.min(max_scroll);

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
            "  Ctrl+C       Quit".to_string(),
            "  ?            Toggle this help".to_string(),
            "".to_string(),
            "  VIM-STYLE NAVIGATION".to_string(),
            "  h            Move cursor left (in input)".to_string(),
            "  j            Scroll down".to_string(),
            "  k            Scroll up".to_string(),
            "  l            Move cursor right (in input)".to_string(),
            "  g            Go to top".to_string(),
            "  G            Go to bottom".to_string(),
            "".to_string(),
            "  SCROLLING".to_string(),
            "  Up/Down      Scroll messages".to_string(),
            "  PageUp       Scroll up one page".to_string(),
            "  PageDown     Scroll down one page".to_string(),
            "  u            Scroll up half page".to_string(),
            "  d            Scroll down half page".to_string(),
            "".to_string(),
            "  COMMAND HISTORY".to_string(),
            "  Ctrl+R       Search history (matches current input)".to_string(),
            "  Ctrl+Up      Previous command".to_string(),
            "  Ctrl+Down    Next command".to_string(),
            "".to_string(),
            "  TEXT EDITING".to_string(),
            "  Left/Right   Move cursor".to_string(),
            "  Home/End     Jump to start/end".to_string(),
            "  Backspace    Delete backwards".to_string(),
            "  Delete       Delete forwards".to_string(),
            "".to_string(),
            "  AGENTS".to_string(),
            "  ======".to_string(),
            "".to_string(),
            "  build        Full access for development work".to_string(),
            "  plan         Read-only for analysis & exploration".to_string(),
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
