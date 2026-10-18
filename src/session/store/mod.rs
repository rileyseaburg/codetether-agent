//! Transactional indexed storage. JSON is an import/export format only.
//! [`State`] records revisions and loaded-window offsets for delta persistence.
mod api;
pub(crate) mod archive;
mod batch;
mod blobs;
mod connection;
pub(crate) mod constraints;
pub(crate) mod cursor;
mod cursor_read;
pub(crate) mod delete;
mod events;
pub(crate) mod append;
mod header;
pub(crate) mod header_read;
pub(crate) mod id;
mod identity;
pub(crate) mod imported;
mod legacy;
pub(crate) mod listing;
mod load;
mod marker;
mod migration;
pub(crate) mod projection;
mod read;
mod records;
mod restore;
mod resume;
mod save;
mod snapshot;
mod state;
pub(crate) mod summaries;
mod summary_import;
mod tool_state;
mod transaction;
pub(crate) use load::load;
pub(crate) use save::save;
pub use state::State;
#[cfg(test)]
mod tests;

/// Maximum messages retained by interactive/background readers.
pub use codetether_session_store::window::WINDOW;
