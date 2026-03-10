use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_terminal_info(f: &mut Frame, area: Rect) {
    let widget = Paragraph::new(vec![
        Line::from("Terminal subsystem integrated."),
        Line::from("Alternate screen + raw mode active while TUI runs."),
    ])
    .block(Block::default().borders(Borders::ALL).title("Terminal"));
    f.render_widget(widget, area);
}
