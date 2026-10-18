//! Session event archive replay and file metadata endpoints.
//!
//! replay_session_events reads configured JSONL archives with offset/tool
//! filtering. list_session_event_files returns the matching file index.
mod directory;
mod index;
mod range;
mod read;
mod replay;
mod scan;
mod types;
pub use index::list_session_event_files;
pub use replay::replay_session_events;
pub use types::{EventFileMeta, ReplayQuery};
/// HTTP status and message returned when an archive operation fails.
pub type ReplayError = (axum::http::StatusCode, String);
