//! Conversion of local agent attempts into task outcomes.
//!
//! [from_agent] applies shared deliverable policy and constructs the task result
//! and terminal status using host-supplied [ExecutionSpec] and elapsed duration.
use crate::{
    SubTaskResult, SubTaskStatus,
    agent_retry::{AgentLoopExit, AgentOutcome},
    deliverable,
};
use std::time::Duration;
mod spec;
pub use spec::ExecutionSpec;
mod build;
pub use build::from_agent;
mod make;
mod success;
