//! Model picker state, catalog filtering and terminal presentation.
//!
//! [matching] implements the shared query policy. Supply a [PickerView] to
//! [render_model_picker] to display its filtered catalog and current selection.
//! [ModelPickerState] owns catalog, query, selection and lifecycle flags;
//! the application retains provider refresh and shared status output.
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

mod state;
mod state_catalog;
mod state_lifecycle;
mod state_navigation;
pub use state::ModelPickerState;
