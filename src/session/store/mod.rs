//! Transactional indexed storage. JSON is an import/export format only.
//! [`State`] records revisions and loaded-window offsets for delta persistence.
pub(crate) mod archive;
mod batch;
use codetether_session_store::blobs;
use codetether_session_store::connection;
pub(crate) mod constraints;
pub(crate) mod cursor;
#[cfg(test)]
use codetether_session_store::consumer as cursor_read;
pub(crate) use codetether_session_store::delete;
use codetether_session_store::events;
pub(crate) use codetether_session_store::header_read;
pub(crate) use codetether_session_store::id;
use codetether_session_store::identity;
pub(crate) mod imported;
codetether_session_listing::impl_session_header!(crate::session::Session);
mod load;
use codetether_session_store::marker;
mod migration;
pub(crate) use codetether_session_store::projection;
use codetether_session_store::read;
use codetether_session_store::records;
mod save;
mod snapshot;
mod state;
pub(crate) use codetether_session_store::summaries;
use codetether_session_store::tool_state;
use codetether_session_store::transaction;
pub(crate) use load::load;
pub(crate) use save::save;
pub use state::State;
#[cfg(test)]
mod tests;

/// Maximum messages retained by interactive/background readers.
pub use codetether_session_store::window::WINDOW;
