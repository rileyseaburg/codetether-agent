//! Model picker selection presentation.
use crate::PickerView;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

pub(super) fn render(f: &mut Frame, view: &PickerView<'_>, models: &[&str], area: Rect) {
    let current = models
        .get(view.selected_index)
        .copied()
        .or(view.session_model)
        .unwrap_or("auto");
    let current_widget = Paragraph::new(Line::from(vec![
        Span::raw("Current selection: "),
        Span::styled(current.to_string(), Style::default().fg(Color::Cyan).bold()),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Selection "));
    f.render_widget(current_widget, area);
}
