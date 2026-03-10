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
        Line::from("  /help            Help"),
        Line::from("  /sessions        Sessions"),
        Line::from("  /swarm           Swarm"),
        Line::from("  /ralph           Ralph"),
        Line::from("  /bus             Bus log"),
        Line::from("  /settings        Settings"),
        Line::from("  /lsp             LSP"),
        Line::from("  /rlm             RLM"),
        Line::from("  /symbols         Symbol search"),
        Line::from("  /chat            Back to chat"),
        Line::from("  Esc              Back / exit detail"),
        Line::from(""),
        Line::from("Chat input:"),
        Line::from("  Enter            Send prompt or run slash command"),
        Line::from("  Left/Right       Move cursor"),
        Line::from("  Ctrl+Left/Right  Move by word"),
        Line::from("  Home/End         Jump to start/end"),
        Line::from("  Backspace/Delete Remove text"),
        Line::from("  Up/Down          Scroll transcript"),
        Line::from("  PgUp/PgDn        Faster transcript scroll"),
        Line::from(""),
        Line::from("Sessions / Bus / Swarm / Ralph:"),
        Line::from("  Up/Down          Select item"),
        Line::from("  Enter            Open detail / load selected item"),
        Line::from("  PgUp/PgDn        Scroll detail"),
    ];

    let widget = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_terminal_info(f, chunks[1]);
}
