//! Runtime task execution, stage-state access and successful-story merge bindings.
//!
//! Dedicated task and stage crates own execution and coordination. The host
//! scheduler trait remains available, and merge callers borrow their context.
use super::*;
mod driver;
mod host;
mod scheduling_parallel;
mod stage_host;
