//! Model catalog filtering and terminal picker presentation.
//!
//! [matching] implements the shared query policy. Supply a [PickerView] to
//! [render_model_picker] to display its filtered catalog and current selection.
mod data;
mod filter;
mod footer;
mod header;
mod list;
mod render;
mod selection;
pub use data::PickerView;
pub use filter::matching;
pub use render::render_model_picker;
