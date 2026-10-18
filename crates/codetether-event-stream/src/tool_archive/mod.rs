//! Nonblocking, best-effort archival of completed tool results.
//!
//! Pass a borrowed [Record] to [write]. The writer checks the configured archive
//! path, serializes an event, and spawns an append task. File errors are ignored,
//! matching the prompt loop's existing archival behavior.
mod append;
mod record;
mod write;
pub use record::Record;
pub use write::write;
