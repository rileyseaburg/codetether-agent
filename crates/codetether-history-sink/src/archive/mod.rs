//! Coalesced archival of durable consumer ranges.
//!
//! Implement [ArchiveSource] using the existing session consumer, then call
//! [schedule]. Upload success precedes materialized-view commit and cursor
//! acknowledgement. Failed uploads keep the unacknowledged durable range.
mod drain;
mod host;
mod queue;
mod schedule;
pub use host::ArchiveSource;
pub use schedule::schedule;
