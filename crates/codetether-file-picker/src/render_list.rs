//! File browser list panel.
use super::{image::is_image_file, render_style::entry_style};
use super::{render_chrome::block, types::FilePickerState};
use ratatui::{prelude::*, widgets::*};

pub(crate) fn render_list(f: &mut ratatui::Frame, area: Rect, state: &FilePickerState) {
    let rows = area.height.saturating_sub(2) as usize;
    let (start, end) = super::render_window::bounds(state.selected, state.entries.len(), rows);
    let items = state.entries[start..end]
        .iter()
        .enumerate()
        .map(|(offset, e)| {
            let i = start + offset;
            let style = entry_style(state, i, e.is_dir, is_image_file(&e.path));
            ListItem::new(Line::from(Span::styled(&e.name, style)))
        });
    let title = format!(
        " {} [{}] {}/{} ",
        state.dir.display(),
        state.filter,
        state.selected.saturating_add(1).min(state.entries.len()),
        state.entries.len()
    );
    f.render_widget(List::new(items).block(block(title)), area);
}
