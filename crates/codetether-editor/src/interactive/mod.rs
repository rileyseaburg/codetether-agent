//! Embedded editor opening and fuzzy file selection.
//!
//! Implement [`View`] for client state. [`open_argument`] handles an `/edit`
//! argument; [`open_finder`] and [`confirm_finder`] manage workspace selection.
mod finder;
mod open;
mod view;
pub use finder::{confirm_finder, open_finder};
pub use open::{open_argument, open_path};
pub use view::View;
