use ratatui::prelude::*;

use super::types::FilePickerState;

pub fn entry_style(state: &FilePickerState, i: usize, dir: bool, image: bool) -> Style {
    if i == state.selected {
        Style::default().fg(Color::Cyan).bold()
    } else if dir {
        Style::default().fg(Color::Blue)
    } else if image {
        Style::default().fg(Color::Magenta)
    } else {
        Style::default().fg(Color::Gray)
    }
}
