//! Embedded editor opening and fuzzy file selection.
//!
//! Implement [`View`] for client state. [`open_argument`] handles an `/edit`
//! argument; [`open_finder`] and [`confirm_finder`] manage workspace selection.
mod binding;
mod finder;
mod open;
mod view;
pub use finder::{confirm_finder, open_finder};
pub use open::{open_argument, open_path};
pub use view::View;

mod key_host;
mod keys;
pub use key_host::KeyHost;
pub use keys::handle_editor_key;

mod finder_keys;
mod finder_view;
pub use finder_keys::handle_finder_key;
pub use finder_view::FinderView;
