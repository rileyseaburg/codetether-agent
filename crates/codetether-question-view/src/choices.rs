//! Choice rows and scrolling selection.
use codetether_question::Draft;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListItem, ListState},
};

pub(super) fn render(frame: &mut Frame, area: Rect, draft: &Draft) {
    let q = draft.question();
    let mut items: Vec<_> = q
        .options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            let marker = if q.multiple {
                if draft.selected()[i] { "[x]" } else { "[ ]" }
            } else {
                "○"
            };
            ListItem::new(vec![
                Line::from(format!("{marker} {}", option.label)),
                Line::from(format!("    {}", option.description)).dim(),
            ])
        })
        .collect();
    items.push(ListItem::new(format!(
        "Type your own answer: {}",
        draft.custom()
    )));
    let list = List::new(items)
        .highlight_symbol("› ")
        .highlight_style(Style::new().cyan().bold());
    let mut selected = ListState::default().with_selected(Some(draft.cursor()));
    frame.render_stateful_widget(list, area, &mut selected);
}
