use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::status::render_status;

pub fn render_settings(f: &mut Frame, area: Rect, status: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let lines = vec![
        Line::from("Settings"),
        Line::from(""),
        Line::from("This panel is now integrated into the TUI shell."),
        Line::from("Future settings can live here:"),
        Line::from("  • default provider/model"),
        Line::from("  • theme/color mode"),
        Line::from("  • worker bridge defaults"),
        Line::from("  • session behavior"),
        Line::from(""),
        Line::from("Use Esc to return to chat."),
    ];

    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Settings"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_status(f, chunks[1], status);
}
