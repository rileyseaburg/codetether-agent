//! Model picker list presentation.
use crate::PickerView;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

pub(super) fn render(f: &mut Frame, view: &PickerView<'_>, models: &[&str], area: Rect) {
    let items: Vec<ListItem<'static>> = if models.is_empty() {
        let empty_label = if view.refreshing {
            "Loading models..."
        } else {
            "No models available"
        };
        vec![ListItem::new(empty_label)]
    } else {
        models
            .iter()
            .map(|model| ListItem::new((*model).to_string()))
            .collect()
    };

    let mut list_state = ListState::default();
    if !models.is_empty() {
        list_state.select(Some(view.selected_index.min(models.len() - 1)));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Available Models "),
        )
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold())
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, area, &mut list_state);
}
