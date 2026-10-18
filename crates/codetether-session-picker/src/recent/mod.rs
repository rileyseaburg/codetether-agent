//! Compact recent-session panel for sidebar layouts.
//!
//! [render] displays a bounded window around the selected session, using the
//! same [crate::SessionPickerState] as the full session picker.
mod render;
mod rows;
pub use render::render;
