//! Read-only discovery and metadata summaries of Codex rollout archives.
//!
//! [discover_codex_sessions_for_directory] scans the configured archive home.
//! [discover_codex_sessions_in] accepts an explicit archive for callers and tests.
//! [read_session_meta] and [load_session_index] are shared with full importers.
mod candidate;
mod discover;
mod find;
mod home;
mod index;
mod info;
mod meta;
mod meta_first;
mod meta_recover;
mod paths;
mod reasoning_count;
mod scan_state;
mod summary;
mod summary_count;
mod summary_scan;
mod summary_text;
mod title;
pub use discover::{discover_codex_sessions_for_directory, discover_codex_sessions_in};
pub use find::find_codex_session_path_by_id;
pub use home::{codex_home_dir, codex_home_dir_for_path};
pub use index::load_session_index;
pub use info::CodexSessionInfo;
pub use meta::read_session_meta;
pub use summary::summarize_codex_session_from_path;
pub use title::derive_title_from_text;
