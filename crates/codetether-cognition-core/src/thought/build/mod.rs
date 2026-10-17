//! Build thought events and distilled memory snapshots from completed work.
//!
//! [thought_event] preserves phase-specific event payloads. [build_snapshot]
//! summarizes a completed thought using [metadata] for provenance.
mod event;
mod metadata;
mod snapshot;
pub use event::thought_event;
pub use metadata::metadata;
pub use snapshot::build_snapshot;
