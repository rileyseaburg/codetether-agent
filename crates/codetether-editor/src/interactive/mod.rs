//! Embedded editor opening and fuzzy file selection.
//!
//! Implement [`View`] for client state. [`open_argument`] handles an `/edit`
//! argument; [`open_finder`] and [`confirm_finder`] manage workspace selection.
//! [`EditorView`] supplies the view traits from borrowed fields and a navigation
//! callback when the frontend does not need a custom implementation.
mod binding;
mod borrowed_view;
mod finder;
mod open;
mod view;
pub use borrowed_view::EditorView;
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
