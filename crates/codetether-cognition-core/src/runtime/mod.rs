//! Shared cognition state, restored data and loop-handle snapshots.
//!
//! [CognitionRuntime] owns all shared runtime stores and normalized limits.
//! [InitialState] carries restored content. [LoopContext] clones the handles
//! required by the background loop without owning application service types.
mod assemble;
mod initial;
mod loop_build;
mod loop_context;
mod state;
pub use initial::InitialState;
pub use loop_context::LoopContext;
pub use state::CognitionRuntime;

mod approval;
mod lineage;
mod query;
mod status;

mod create;
mod start;
mod stop;

/// One tick of cognition work, governance and publication.
pub mod tick;

/// Per-work-item cognition phase execution and state changes.
pub mod process;

mod reap;
mod spawn;
