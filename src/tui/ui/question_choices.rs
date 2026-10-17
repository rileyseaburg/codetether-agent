//! Choice list scrolls the current selection into view on small terminals.
use codetether_question::Draft;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{List, ListItem, ListState, Paragraph, Wrap},
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

pub(super) fn help(frame: &mut Frame, area: Rect, multiple: bool) {
    let help = if multiple {
        "↑↓/Tab move · Space toggle · Enter next/submit\nType custom answer · Shift+Tab back · Esc dismiss · Ctrl+C cancel turn"
    } else {
        "↑↓/Tab move · Enter choose/next\nType custom answer · Shift+Tab back · Esc dismiss · Ctrl+C cancel turn"
    };
    frame.render_widget(Paragraph::new(help).dim().wrap(Wrap { trim: true }), area);
}
