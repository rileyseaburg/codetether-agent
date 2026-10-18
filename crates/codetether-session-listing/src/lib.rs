//! Cache-aware listing of native and legacy session records.
//!
//! [Source] supplies the application's directory and indexed database listing.
//! [list_sessions], [list_sessions_for_directory] and [list_sessions_paged]
//! combine those rows with cached legacy-file metadata. [merge_summary]
//! preserves the existing precedence for imported session summaries.
mod cache;
mod cache_io;
mod count_seq;
mod directory;
mod merge;
mod parse;
mod path;
mod record;
mod resolve;
mod scan;
mod source;
mod workspace;
use codetether_session_metadata as summary;
pub use directory::{list_sessions, list_sessions_for_directory, list_sessions_paged};
pub use merge::merge_summary;
pub use scan::scan;
pub use source::Source;
