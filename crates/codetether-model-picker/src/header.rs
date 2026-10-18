//! Model picker header presentation.
use crate::PickerView;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub(super) fn render(f: &mut Frame, view: &PickerView<'_>, area: Rect) {
    let filter_suffix = if view.filter.is_empty() {
        String::new()
    } else {
        format!(" [filter: {}]", view.filter)
    };

    let header = Paragraph::new(vec![Line::from(vec![
        Span::raw(" Model picker ").black().on_cyan(),
        Span::raw(" "),
        Span::raw(view.status.to_string()).dim(),
    ])])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Models{} ", filter_suffix)),
    );
    f.render_widget(header, area);
}
