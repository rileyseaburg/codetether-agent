use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_status(f: &mut Frame, area: Rect, text: &str) {
    let widget = Paragraph::new(text.to_string())
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(widget, area);
}
