//! Native task execution, stage-state access and successful-story merge bindings.
//!
//! Dedicated task and stage crates own execution and coordination. The native
//! scheduler trait remains available, and merge callers borrow their context.
use super::*;
mod driver;
mod host;
pub(super) mod parallel_result_types;
mod scheduling_parallel;
mod stage_host;
