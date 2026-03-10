use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_sessions_summary(f: &mut Frame, area: Rect, count: usize, selected: usize) {
    let lines = vec![
        Line::from("Workspace Sessions Summary"),
        Line::from(""),
        Line::from(format!("Sessions found: {count}")),
        Line::from(format!("Selected index: {selected}")),
        Line::from(""),
        Line::from("Press F2 to open the full session picker from chat."),
    ];

    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Sessions"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, area);
}
