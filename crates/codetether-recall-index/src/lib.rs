//! Compact recall indexing and local ranking without native session storage.
//!
//! Build an [IndexedSession] from a [ProjectionInput], prepare a [Query],
//! then merge its scored hits with [merge]. Excerpts and embeddings retain
//! transcript provenance. Native callers own paths, snapshots and sidecar I/O.
mod build;
mod build_document;
mod document;
mod excerpt;
mod indexed_session;
mod input;
pub mod part;
mod part_result;
mod query;
mod rank;
mod rank_score;
pub mod tokens;
mod truncate;
pub use build::project;
pub use codetether_session_recall::RecallHit;
pub use document::RecallDocument;
pub use indexed_session::{IndexedSession, SCHEMA_VERSION};
pub use input::ProjectionInput;
pub use query::Query;
pub use rank::{merge, search};
#[cfg(test)]
mod tests;

/// Implementation support for application-local session bindings.
#[doc(hidden)]
pub mod session_binding;
