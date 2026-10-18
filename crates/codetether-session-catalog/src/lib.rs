//! Combined catalog of native sessions and Codex archive summaries.
//!
//! [list_all_sessions_for_directory] combines both sources using a native
//! [codetether_session_listing::Source]. [list_codex_sessions_for_directory]
//! exposes the archive-only view with the existing Codex title prefix.
mod codex;
mod combined;
mod imported;
mod merge;
pub use codex::list_codex_sessions_for_directory;
pub use combined::list_all_sessions_for_directory;
