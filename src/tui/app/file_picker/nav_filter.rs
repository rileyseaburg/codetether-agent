use crate::tui::app::state::App;

use super::filter::rescan_with_filter;
use super::types::FilePickerMode;

pub fn file_picker_filter_backspace(app: &mut App) {
    if app.state.file_picker.mode == FilePickerMode::Browse {
        app.state.file_picker.filter.pop();
        rescan_with_filter(&mut app.state.file_picker);
    }
}

pub fn file_picker_filter_push(app: &mut App, c: char) {
    if app.state.file_picker.mode == FilePickerMode::Browse {
        app.state.file_picker.filter.push(c);
        rescan_with_filter(&mut app.state.file_picker);
    }
}
