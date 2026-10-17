//! Persistent child-process sessions with incremental output and liveness.
//!
//! [Registry] owns [Running] commands between polls. Build a [Running] value
//! from a configured child and [SpawnMetadata], then call its poll/write methods.
//! [tool_result] converts a [Poll] into the shared model-facing tool response.
mod activity;
mod buffer;
mod drain;
mod readers;
mod registry;
mod result;
mod running;
mod types;
pub use registry::Registry;
pub use result::tool_result;
pub use running::Running;
pub use types::{Poll, SpawnMetadata};

mod spawn;
pub use spawn::spawn_direct;

mod create;
pub use create::spawn;
