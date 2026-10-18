//! Model picker state, catalog filtering and terminal presentation.
//!
//! [matching] implements the shared query policy. Supply a [PickerView] to
//! [render_model_picker] to display its filtered catalog and current selection.
//! [ModelPickerState] owns catalog, query, selection and lifecycle flags;
//! it also owns refresh completion and cache updates. The application supplies
//! a provider registry and retains shared status output.
mod data;
mod feedback;
mod filter;
mod footer;
mod header;
pub mod interactive;
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

mod state_store;

mod refresh_direct;
mod refresh_drain;
mod refresh_finish;
mod refresh_start;

#[cfg(test)]
mod refresh_tests;

/// Application-local state method declarations.
pub mod state_binding;
