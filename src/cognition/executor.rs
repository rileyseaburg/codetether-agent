//! Shared execution contracts and live application tool dispatch.
use super::ThoughtEvent;
use super::thinker::ThinkerClient;
use crate::tool::ToolRegistry;
use codetether_cognition_core::execution::requests;
pub use codetether_cognition_core::execution::{
    CapabilityLease, DecisionReceipt, ExecutionOutcome, ToolInvocation,
};
use std::sync::Arc;
#[path = "executor/extract.rs"]
mod extract;
#[path = "executor/run.rs"]
mod run;
pub use extract::execute_tool_requests;
#[cfg(test)]
#[path = "executor/tests.rs"]
mod tests;
