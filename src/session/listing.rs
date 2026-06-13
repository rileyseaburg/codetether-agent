//! Session listing facade.
//!
//! Listing flows through an append-only `index.jsonl` so we do not have
//! to open every session file just to render a list:
//! - [`index_file`] resolves the path and triggers cheap appends.
//! - [`index_file_io`] owns the blocking read/append I/O.
//! - [`index_file_compact`] atomically rewrites the index when it
//!   becomes bloated or when we rebuild from a full scan.
//! - [`scan`] is the full-directory rebuild/repair path.
//! - [`parse_summary`] is the per-file header parser.
//! - [`directory`] is the public entry point and tries the index first.

mod count_seq;
mod directory;
mod directory_query;
mod disk_walk;
pub(crate) mod index_file;
mod index_file_compact;
mod index_file_compact_io;
mod index_file_io;
mod index_prune;
mod merge_disk;
mod parse_summary;
mod rebuild;
mod record;
mod scan;
mod summary;
mod workspace;
mod workspace_query;

#[cfg(test)]
mod tests;

pub use directory::{list_sessions, list_sessions_for_directory, list_sessions_paged};
pub use summary::SessionSummary;
