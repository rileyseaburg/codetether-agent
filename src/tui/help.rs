use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::terminal::render_terminal_info;

pub fn render_help(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(4)])
        .split(area);

    let text = vec![
        Line::from("CodeTether TUI Help"),
        Line::from(""),
        Line::from("Global:"),
        Line::from("  Ctrl+C / Ctrl+Q  Quit"),
        Line::from("  F1               Help"),
        Line::from("  F2               Sessions"),
        Line::from("  F3               Swarm"),
        Line::from("  F4               Ralph"),
        Line::from("  F5               Bus log"),
        Line::from("  F6               Settings"),
        Line::from("  F7               LSP"),
        Line::from("  F8               RLM"),
        Line::from("  Esc              Back to chat / exit detail"),
        Line::from(""),
        Line::from("Chat input:"),
        Line::from("  Enter            Send prompt"),
        Line::from("  Left/Right       Move cursor"),
        Line::from("  Ctrl+Left/Right  Move by word"),
        Line::from("  Home/End         Jump to start/end"),
        Line::from("  Backspace/Delete Remove text"),
        Line::from("  Up/Down          Scroll transcript"),
        Line::from("  PgUp/PgDn        Faster transcript scroll"),
        Line::from(""),
        Line::from("Sessions / Bus / Swarm / Ralph:"),
        Line::from("  Up/Down          Select item"),
        Line::from("  Enter            Open detail"),
        Line::from("  PgUp/PgDn        Scroll detail"),
    ];

    let widget = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_terminal_info(f, chunks[1]);
}
