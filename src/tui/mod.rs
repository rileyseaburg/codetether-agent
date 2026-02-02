//! Terminal User Interface
//!
//! Interactive TUI using Ratatui

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::path::PathBuf;

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

/// Application state
struct App {
    input: String,
    cursor_position: usize,
    messages: Vec<ChatMessage>,
    current_agent: String,
    scroll: usize,
    show_help: bool,
}

struct ChatMessage {
    role: String,
    content: String,
    timestamp: String,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            messages: vec![ChatMessage {
                role: "system".to_string(),
                content: "Welcome to CodeTether Agent! Type a message to get started, or press ? for help.".to_string(),
                timestamp: chrono::Local::now().format("%H:%M").to_string(),
            }],
            current_agent: "build".to_string(),
            scroll: 0,
            show_help: false,
        }
    }

    fn submit_message(&mut self) {
        if self.input.is_empty() {
            return;
        }

        let message = std::mem::take(&mut self.input);
        self.cursor_position = 0;

        // Add user message
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: message.clone(),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
        });

        // TODO: Actually process the message with the agent
        self.messages.push(ChatMessage {
            role: "assistant".to_string(),
            content: format!("[Agent processing not yet implemented]\n\nYou said: {}", message),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
        });
    }
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

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
                        app.submit_message();
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

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),      // Messages
            Constraint::Length(3),   // Input
            Constraint::Length(1),   // Status bar
        ])
        .split(f.area());

    // Messages area
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" CodeTether Agent [{}] ", app.current_agent))
        .border_style(Style::default().fg(Color::Cyan));

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let style = match m.role.as_str() {
                "user" => Style::default().fg(Color::Green),
                "assistant" => Style::default().fg(Color::Blue),
                "system" => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            };

            let header = Line::from(vec![
                Span::styled(format!("[{}] ", m.timestamp), Style::default().fg(Color::DarkGray)),
                Span::styled(&m.role, style.add_modifier(Modifier::BOLD)),
            ]);

            let content = Line::from(Span::raw(&m.content));

            ListItem::new(vec![header, content, Line::from("")])
        })
        .collect();

    let messages_list = List::new(messages).block(messages_block);
    f.render_widget(messages_list, chunks[0]);

    // Input area
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(" Message (Enter to send) ")
        .border_style(Style::default().fg(Color::White));

    let input = Paragraph::new(app.input.as_str())
        .block(input_block)
        .wrap(Wrap { trim: false });
    f.render_widget(input, chunks[1]);

    // Cursor
    f.set_cursor_position((
        chunks[1].x + app.cursor_position as u16 + 1,
        chunks[1].y + 1,
    ));

    // Status bar
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".to_string());

    let status = Paragraph::new(Line::from(vec![
        Span::styled(" ? ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Help  "),
        Span::styled(" Tab ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Switch Agent  "),
        Span::styled(" Ctrl+C ", Style::default().fg(Color::Black).bg(Color::White)),
        Span::raw(" Quit  "),
        Span::styled(format!(" {} ", cwd), Style::default().fg(Color::DarkGray)),
    ]));
    f.render_widget(status, chunks[2]);

    // Help overlay
    if app.show_help {
        let area = centered_rect(60, 60, f.area());
        f.render_widget(Clear, area);

        let help_text = vec![
            "",
            "  KEYBOARD SHORTCUTS",
            "  ==================",
            "",
            "  Enter        Send message",
            "  Tab          Switch between build/plan agents",
            "  Ctrl+C       Quit",
            "  ?            Toggle this help",
            "",
            "  Up/Down      Scroll messages",
            "  PageUp/Down  Scroll faster",
            "",
            "  AGENTS",
            "  ======",
            "",
            "  build        Full access for development work",
            "  plan         Read-only for analysis & exploration",
            "",
            "  Press ? or Esc to close",
            "",
        ];

        let help = Paragraph::new(help_text.join("\n"))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Help ")
                    .border_style(Style::default().fg(Color::Yellow)),
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
