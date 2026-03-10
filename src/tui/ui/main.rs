use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, Cell, Row, Wrap,
    },
    Frame,
};
use crate::tui::app::state::*;
use crate::tui::chat::message::*;
use crate::tui::models::*;
use crate::tui::constants::*;

pub fn ui(f: &mut Frame, app: &mut App) {
    // Placeholder rendering for debugging
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(area);

    f.render_widget(
        Block::default().borders(Borders::ALL).title("TUI Refactor in Progress"),
        chunks[0]
    );
    
    f.render_widget(
        Paragraph::new(app.state.input.as_str())
            .block(Block::default().borders(Borders::ALL).title("Input")),
        chunks[1]
    );
}
