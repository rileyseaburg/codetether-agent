//! File browser and read-only content viewer for TUI attachments.

mod host;
mod nav_attach;
mod open;
mod render;
use codetether_file_picker as types;

#[cfg(test)]
mod tests;

pub use codetether_file_picker::{file_picker_enter, file_picker_escape};
pub use codetether_file_picker::{file_picker_filter_backspace, file_picker_filter_push};
pub use codetether_file_picker::{
    file_picker_page_down, file_picker_page_up, file_picker_select_next, file_picker_select_prev,
};
pub use codetether_file_picker::{file_picker_select_first, file_picker_select_last};
pub use nav_attach::file_picker_attach;
pub use open::{open_file_in_viewer, open_file_picker};
pub use render::render_file_picker;
pub use types::{FilePickerEntry, FilePickerMode, FilePickerState};
