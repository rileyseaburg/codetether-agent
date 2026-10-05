//! Transactional indexed storage. JSON is an import/export format only.
//! [`State`] records revisions and loaded-window offsets for delta persistence.
mod api;
pub(crate) mod archive;
mod batch;
mod blobs;
mod checkpoint;
mod connection;
pub(crate) mod constraints;
pub(crate) mod cursor;
mod cursor_read;
pub(crate) mod delete;
mod events;
pub(crate) mod evict;
mod header;
pub(crate) mod header_read;
mod hydrate;
pub(crate) mod id;
mod identity;
pub(crate) mod imported;
pub(crate) mod listing;
mod load;
mod marker;
mod migration;
pub(crate) mod projection;
mod read;
mod records;
mod recover_locator;
mod resume;
mod save;
mod schema;
mod state;
pub(crate) mod summaries;
mod summary_import;
mod tool_state;
mod transaction;
mod upgrade;
pub(crate) use load::load;
pub(crate) use save::save;
pub use state::State;
#[cfg(test)]
mod tests;

/// Maximum messages retained by interactive/background readers.
pub const WINDOW: usize = 1_000;
