//! File references, shortcuts and click actions for chat transcripts.
//!
//! [path_at] resolves path tokens near a click. [latest_shared_file] searches
//! shared-file messages and file tool calls newest-first. [Host] supplies UI
//! effects for diff, viewer and editor actions; parsing and selection remain
//! independent of the application state type.
mod actions;
mod click;
mod extract;
mod host;
mod keys;
mod path;
mod resolve;
pub use actions::{goto_shared_file_definition, show_shared_file_diff};
pub use click::click_open_path;
pub use host::Host;
pub use keys::ctrl_key;
pub use path::path_at;
pub use resolve::latest_shared_file;
