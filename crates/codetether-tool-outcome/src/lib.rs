//! Post-execution confirmation, output rendering and workspace classification.
//!
//! Construct [Context] from the original call and workspace, then call [render]
//! with enriched execution arguments and its raw result. [Outcome] preserves
//! metadata and records timing and empty-search classification for the caller.
mod auto_apply;
mod classify;
mod context;
mod render;
mod types;
pub use context::Context;
pub use render::render;
pub use types::Outcome;
