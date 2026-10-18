//! Session persistence lifecycle, identity validation and notification order.
//!
//! Implement [Host] for native loading and indexing. [load] and [load_tail]
//! validate identity; [save] notifies only after a write; [delete] removes
//! indices, records and locators in the original order.
mod delete;
mod host;
mod latest;
mod load;
mod path;
mod save;
pub use delete::delete;
pub use host::Host;
pub use latest::last_for_directory_tail;
pub use load::{load, load_tail};
pub use path::{session_path, sessions_dir};
pub use save::save;
