//! Terminal view of registered A2A agents and their display cards.
//!
//! [RegistryView] borrows host data; [render_protocol_registry] builds sorted
//! cards and renders the list, capability details and empty state.
mod cards;
mod detail_lines;
mod details;
mod list;
mod list_row;
mod render;
mod text;
mod view;
pub use render::render_protocol_registry;
pub use view::RegistryView;
