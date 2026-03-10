use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_input_preview(f: &mut Frame, area: Rect, text: &str) {
    let widget = Paragraph::new(text.to_string())
        .block(Block::default().borders(Borders::ALL).title("Input Preview"));
    f.render_widget(widget, area);
}
