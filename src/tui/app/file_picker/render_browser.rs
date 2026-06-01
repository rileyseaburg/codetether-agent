use ratatui::{prelude::*, widgets::*};

use super::image::is_image_file;
use super::render_chrome::block;
use super::render_style::entry_style;
use super::types::FilePickerState;

pub fn render_browser(f: &mut ratatui::Frame, area: Rect, state: &FilePickerState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(area);
    render_list(f, chunks[0], state);
    render_preview(f, chunks[1], state);
}

fn render_list(f: &mut ratatui::Frame, area: Rect, state: &FilePickerState) {
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

fn render_preview(f: &mut ratatui::Frame, area: Rect, state: &FilePickerState) {
    let lines = state
        .preview
        .as_ref()
        .map(|preview| preview.lines.clone())
        .unwrap_or_else(|| vec!["No file selected".to_string()]);
    let widget = Paragraph::new(lines.join("\n"))
        .block(block(" Preview "))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, area);
}
