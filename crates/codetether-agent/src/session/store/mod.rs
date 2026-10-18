//! Transactional indexed storage. JSON is an import/export format only.
//! [`State`] records revisions and loaded-window offsets for delta persistence.
#[cfg(test)]
use codetether_session_store as batch;
pub(crate) mod constraints;
#[cfg(test)]
pub(crate) mod cursor;
#[cfg(test)]
mod load;
#[cfg(test)]
use codetether_session_store::consumer as cursor_read;
pub(crate) use codetether_session_store::legacy as imported;
codetether_session_listing::impl_session_header!(crate::session::Session);
#[cfg(test)]
mod migration;
mod native;
mod snapshot;
mod state;
pub use state::State;
#[cfg(test)]
mod tests;

/// Maximum messages retained by interactive/background readers.
pub use codetether_session_store::window::WINDOW;

#[cfg(test)]
include!("mod_test_imports.rs");
