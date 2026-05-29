use ratatui::{prelude::*, widgets::*};

use super::render_chrome::block;
use super::types::FilePickerState;

pub fn render_viewer(f: &mut ratatui::Frame, area: Rect, state: &FilePickerState) {
    let Some(preview) = &state.preview else {
        return;
    };
    let mut lines = preview.lines.clone();
    if preview.truncated {
        lines.push("[preview truncated]".to_string());
    }
    if preview.binary {
        lines.push("[binary content hidden]".to_string());
    }
    let text = lines
        .into_iter()
        .skip(state.preview_scroll)
        .collect::<Vec<_>>()
        .join("\n");
    let title = format!(" {} ", preview.path.display());
    f.render_widget(Paragraph::new(text).block(block(title)), area);
}
