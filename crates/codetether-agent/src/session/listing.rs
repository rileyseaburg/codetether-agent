//! Session listing API backed by the shared listing crate.
mod directory;
#[cfg(test)]
mod scan;
use codetether_session_metadata as summary;
#[cfg(test)]
mod tests;
pub use directory::{list_sessions, list_sessions_for_directory, list_sessions_paged};
pub(super) type ApplicationSource =
    codetether_session_listing::indexed::IndexedSource<super::Session>;
pub use summary::SessionSummary;
