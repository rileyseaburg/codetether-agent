//! Transactional indexed storage. JSON is an import/export format only.
//! [`State`] records revisions and loaded-window offsets for delta persistence.
mod api;
pub(crate) mod archive;
mod batch;
use codetether_session_store::blobs;
use codetether_session_store::connection;
pub(crate) mod constraints;
pub(crate) mod cursor;
#[cfg(test)]
use codetether_session_store::consumer as cursor_read;
pub(crate) mod append;
pub(crate) use codetether_session_store::delete;
use codetether_session_store::events;
mod header;
pub(crate) use codetether_session_store::header_read;
pub(crate) use codetether_session_store::id;
use codetether_session_store::identity;
pub(crate) mod imported;
mod legacy;
pub(crate) mod listing;
mod load;
use codetether_session_store::marker;
mod migration;
pub(crate) mod projection;
use codetether_session_store::read;
use codetether_session_store::records;
mod restore;
mod resume;
mod save;
mod snapshot;
mod state;
pub(crate) mod summaries;
use codetether_session_store::tool_state;
use codetether_session_store::transaction;
pub(crate) use load::load;
pub(crate) use save::save;
pub use state::State;
#[cfg(test)]
mod tests;

/// Maximum messages retained by interactive/background readers.
pub use codetether_session_store::window::WINDOW;
