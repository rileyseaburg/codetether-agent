//! Session picker filtering, row formatting, and terminal presentation.
//!
//! Construct a borrowed [PickerView] and call [render_sessions_view].
//! Application loading and bus status remain with the caller.
mod age_color;
mod filter;
mod header;
mod help;
mod input_binding;
mod item;
mod list;
mod list_item;
pub mod refresh;
mod relative_time;
mod render;
mod row;
mod view;
mod window;
pub use age_color::age_color;
pub use filter::filtered_sessions;
pub use list_item::session_list_item;
pub use render::render_sessions_view;
pub use row::session_row_summary;
pub use view::PickerView;
pub use window::bounds;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod row_tests;

mod state;
mod state_filter;
mod state_listing;
mod state_nav;
pub use state::SessionPickerState;

mod label;
pub use label::session_label;

/// Compact recent-session sidebar rendering.
pub mod recent;

/// Selection loading and identity-gated continuation.
pub mod interaction;
mod state_selection;
pub mod summary;

/// Application-local state method declarations.
pub mod state_binding;
