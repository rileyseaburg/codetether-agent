//! Proactive context orchestration independent of native session storage.
//!
//! [register] and [prepare] select runtime settings and schedule coalesced
//! work. [prepared_index] prefers durable projections and validates legacy
//! prepared sidecars. [remove] clears runtime registration before storage.
mod host;
mod lifecycle;
mod merge;
mod projection;
mod runtime;
mod source;
pub use host::Host;
pub use lifecycle::{prepare, register, remove, schedule};
pub use merge::prepared_index;
pub use source::Source;

/// First-turn LSP diagnostic and symbol prefetch.
pub mod lsp;
