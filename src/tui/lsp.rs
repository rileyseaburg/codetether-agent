use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::input::render_input_preview;

pub fn render_lsp(f: &mut Frame, area: Rect, cwd: &str, status: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let lines = vec![
        Line::from("LSP Workspace View"),
        Line::from(""),
        Line::from(format!("Workspace: {cwd}")),
        Line::from(format!("Status: {status}")),
        Line::from(""),
        Line::from("Integrated shell for LSP-centric workflows."),
        Line::from("Planned next step: connect symbol search and diagnostics navigation."),
        Line::from("For now, use this view as a dedicated workspace/introspection panel."),
        Line::from(""),
        Line::from("Use F1 for help and Esc to return to chat."),
    ];

    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("LSP"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_input_preview(f, chunks[1], "Symbol search / diagnostics command preview");
}
