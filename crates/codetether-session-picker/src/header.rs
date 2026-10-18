//! Session picker heading and active filter rendering.

use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::PickerView;

pub(super) fn render(f: &mut Frame, view: &PickerView<'_>, area: Rect) {
    let filter = if view.filter.is_empty() {
        String::new()
    } else {
        format!(" [filter: {}]", view.filter)
    };
    let content = Line::from(vec![
        Span::raw(" Session picker ").black().on_cyan(),
        Span::raw(" "),
        Span::raw(view.status.to_string()).dim(),
    ]);
    let title = format!(" Sessions (↑↓ navigate, Enter load, Esc cancel){filter} ");
    let widget = Paragraph::new(content).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(widget, area);
}
