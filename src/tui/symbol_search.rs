//! Symbol search view for the TUI
//!
//! Provides a fuzzy finder-style popup for searching LSP workspace symbols.
//! Triggered by Ctrl+T or /symbols command.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use std::path::PathBuf;

/// A symbol result from LSP workspace/symbol query
#[derive(Debug, Clone)]
pub struct SymbolEntry {
    /// Symbol name (e.g., "LspManager", "run_app")
    pub name: String,
    /// Symbol kind (e.g., "Struct", "Function", "Enum")
    pub kind: String,
    /// File path where the symbol is defined
    pub path: PathBuf,
    /// URI from LSP (e.g., "file:///path/to/file.rs")
    pub uri: Option<String>,
    /// Line number (1-based)
    pub line: Option<u32>,
    /// Container name (e.g., module or parent struct)
    pub container: Option<String>,
}

/// State for the symbol search popup
#[derive(Debug, Default)]
pub struct SymbolSearchState {
    /// Search query string
    pub query: String,
    /// Search results from LSP
    pub results: Vec<SymbolEntry>,
    /// Currently selected result index
    pub selected: usize,
    /// Whether LSP query is in progress
    pub loading: bool,
    /// Error message if LSP failed
    pub error: Option<String>,
    /// List state for ratatui
    pub list_state: ListState,
}

impl SymbolSearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            selected: 0,
            loading: false,
            error: None,
            list_state: ListState::default(),
        }
    }

    /// Clear all state
    pub fn reset(&mut self) {
        self.query.clear();
        self.results.clear();
        self.selected = 0;
        self.loading = false;
        self.error = None;
        self.list_state = ListState::default();
    }

    /// Add a character to the query
    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
        self.selected = 0;
    }

    /// Remove the last character from the query
    pub fn pop_char(&mut self) {
        self.query.pop();
        self.selected = 0;
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        if !self.results.is_empty() {
            self.selected = self.selected.saturating_sub(1);
            self.list_state.select(Some(self.selected));
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.results.is_empty() {
            self.selected = (self.selected + 1).min(self.results.len() - 1);
            self.list_state.select(Some(self.selected));
        }
    }

    /// Get the currently selected symbol
    pub fn selected_symbol(&self) -> Option<&SymbolEntry> {
        self.results.get(self.selected)
    }

    /// Update results from LSP query
    pub fn set_results(&mut self, results: Vec<SymbolEntry>) {
        self.results = results;
        self.selected = 0;
        self.loading = false;
        self.error = None;
        if !self.results.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    /// Set error state
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.loading = false;
        self.results.clear();
    }

    /// Open the symbol search popup
    pub fn open(&mut self) {
        self.reset();
    }

    /// Close the symbol search popup
    pub fn close(&mut self) {
        self.reset();
    }

    /// Handle character input
    pub fn handle_char(&mut self, c: char) {
        self.push_char(c);
    }

    /// Handle backspace
    pub fn handle_backspace(&mut self) {
        self.pop_char();
    }
}

/// Render the symbol search popup overlay
pub fn render_symbol_search(f: &mut Frame, state: &mut SymbolSearchState, area: Rect) {
    // Center the popup
    let popup_area = centered_rect(80, 60, area);

    // Clear the area underneath
    f.render_widget(Clear, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(3),    // Results list
            Constraint::Length(1), // Status/footer
        ])
        .split(popup_area);

    // Search input box
    let input_style = Style::default().fg(Color::Yellow);
    let input = Paragraph::new(Line::from(vec![
        Span::styled("🔍 ", Style::default().fg(Color::Cyan)),
        Span::styled(&state.query, input_style),
        Span::raw("▏"), // Cursor
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Symbol Search (Esc: close, Enter: jump) ")
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(input, chunks[0]);

    // Results list
    let items: Vec<ListItem> = state
        .results
        .iter()
        .map(|sym| {
            let kind_color = symbol_kind_color(&sym.kind);
            let mut spans = vec![
                Span::styled(
                    format!(" {:8} ", sym.kind),
                    Style::default().fg(kind_color),
                ),
                Span::styled(&sym.name, Style::default().fg(Color::White).bold()),
            ];

            if let Some(ref container) = sym.container {
                spans.push(Span::styled(
                    format!(" ({})", container),
                    Style::default().fg(Color::DarkGray),
                ));
            }

            spans.push(Span::styled(
                format!(
                    " → {}",
                    sym.path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| sym.path.display().to_string())
                ),
                Style::default().fg(Color::DarkGray),
            ));

            if let Some(line) = sym.line {
                spans.push(Span::styled(
                    format!(":{}", line),
                    Style::default().fg(Color::Yellow),
                ));
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Results ({}) ", state.results.len()))
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    // Sync list state
    state.list_state.select(Some(state.selected));
    f.render_stateful_widget(list, chunks[1], &mut state.list_state);

    // Status bar
    let status_text = if state.loading {
        Span::styled(" Searching...", Style::default().fg(Color::Yellow))
    } else if let Some(ref err) = state.error {
        Span::styled(format!(" Error: {}", err), Style::default().fg(Color::Red))
    } else if state.results.is_empty() && !state.query.is_empty() {
        Span::styled(" No symbols found", Style::default().fg(Color::DarkGray))
    } else {
        Span::styled(
            " ↑↓:navigate  Enter:open  Esc:close",
            Style::default().fg(Color::DarkGray),
        )
    };

    let status = Paragraph::new(Line::from(status_text));
    f.render_widget(status, chunks[2]);
}

/// Get color for symbol kind
fn symbol_kind_color(kind: &str) -> Color {
    match kind {
        "Function" | "Method" => Color::Yellow,
        "Struct" | "Class" | "Enum" | "Interface" => Color::Cyan,
        "Module" | "Namespace" => Color::Magenta,
        "Constant" | "Field" | "Property" => Color::Green,
        "Variable" | "Parameter" => Color::Blue,
        _ => Color::White,
    }
}

/// Helper to create a centered rectangle
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