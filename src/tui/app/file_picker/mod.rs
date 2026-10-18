//! File browser and read-only content viewer for TUI attachments.

use codetether_file_picker as filter;
mod host;
mod nav;
mod nav_attach;
mod nav_filter;
mod nav_jump;
mod nav_select;
mod open;
use codetether_file_picker as preview;
mod render;
use codetether_file_picker as scan;
mod types;

#[cfg(test)]
mod tests;

pub use nav::{file_picker_enter, file_picker_escape};
pub use nav_attach::file_picker_attach;
pub use nav_filter::{file_picker_filter_backspace, file_picker_filter_push};
pub use nav_jump::{file_picker_select_first, file_picker_select_last};
pub use nav_select::{
    file_picker_page_down, file_picker_page_up, file_picker_select_next, file_picker_select_prev,
};
pub use open::{open_file_in_viewer, open_file_picker};
pub use render::render_file_picker;
pub use types::{FilePickerEntry, FilePickerMode, FilePickerState};
