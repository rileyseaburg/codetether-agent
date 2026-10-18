//! Shared persisted session metadata and lightweight listing records.
//!
//! [SessionMetadata] preserves serialized options and runtime-only provider
//! state while its debug view redacts credentials. [SessionSummary] carries
//! listing fields and delegates safe display labels to the title crate.
mod debug;
mod debug_sink;
mod defaults;
mod labels;
mod model;
mod summary;
pub use model::SessionMetadata;
pub use summary::SessionSummary;

/// Initial session identity and provenance mutation helpers.
pub mod lifecycle;
