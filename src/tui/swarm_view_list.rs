//! SubTask list item construction for the swarm view.
//!
//! Builds one [`ListItem`] per subtask (preserving the index-based selection
//! mapping) while adding a stage-change divider and a WCAG-friendly highlight.

use super::SwarmViewState;
use super::swarm_view_row::task_line;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::ListItem,
};

/// Build list items, prefixing a stage divider whenever the stage changes.
pub(crate) fn subtask_items(state: &SwarmViewState) -> Vec<ListItem<'static>> {
    let mut prev_stage: Option<usize> = None;
    state
        .subtasks
        .iter()
        .map(|task| {
            let show = prev_stage != Some(task.stage);
            prev_stage = Some(task.stage);
            ListItem::new(task_line(task, show))
        })
        .collect()
}

/// Contrast-safe highlight: black text on cyan so the selection is legible
/// for low-vision users (WCAG 1.4.3), unlike bold-on-dark-gray.
pub(crate) fn highlight_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}
