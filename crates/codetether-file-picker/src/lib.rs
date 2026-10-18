//! Directory browsing, bounded previews and file-picker presentation.
//!
//! [FilePickerState] holds the browser/viewer state. [open_file_picker] and
//! [open_file_in_viewer] create it; navigation functions update it and
//! [render_file_picker] renders it. The host owns attachment and view switching.
mod entry;
mod filter;
mod image;
mod mode;
mod nav;
mod nav_filter;
mod nav_jump;
mod nav_select;
mod open_browser;
mod open_viewer;
mod preview;
mod preview_cache;
mod preview_data;
mod read_preview;
mod render;
mod render_browser;
mod render_chrome;
mod render_style;
mod render_viewer;
mod render_window;
mod scan;
mod scan_entry;
mod state;
mod state_default;
mod types;
include!("exports.rs");
#[cfg(test)]
mod tests;

mod binary_lines;
mod render_list;
mod render_preview;

/// Frontend browser opening and attachment workflow.
pub mod interactive;
