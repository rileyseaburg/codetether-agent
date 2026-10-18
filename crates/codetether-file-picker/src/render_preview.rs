//! File browser preview panel.
use super::{render_chrome::block, types::FilePickerState};
use ratatui::{prelude::*, widgets::*};

pub(crate) fn render_preview(f: &mut ratatui::Frame, area: Rect, state: &FilePickerState) {
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
