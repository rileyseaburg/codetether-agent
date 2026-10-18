//! Runtime state and service bindings for sequential story execution.
//!
//! These implementations preserve existing runtime effects without owning
//! story selection, response branching or completion ordering.
use super::*;
mod execution;
mod prompts;
mod reporting;
mod state;
