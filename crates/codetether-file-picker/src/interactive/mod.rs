//! File browser opening and selected-file attachment workflow.
//!
//! [open_file_picker] and [open_file_in_viewer] initialize the frontend.
//! [file_picker_attach] prefers a valid preview path then the selection.
//! [attach_path] loads images or invokes the frontend's text sharing flow.
mod attach;
mod open;
mod select;
mod view;
pub use attach::attach_path;
pub use open::{open_file_in_viewer, open_file_picker};
pub use select::file_picker_attach;
pub use view::View;
