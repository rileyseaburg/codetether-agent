//! Borrowed adapters for forage console, bus and audit reporting.
//!
//! [RunEvents] references the run's existing arguments, rubric and bus handle.
//! Its methods report lifecycle events; execution and loop state stay with callers.
use super::*;
use crate::a2a::types::{Part, TaskState};
use crate::audit::{AuditCategory, AuditOutcome};
use serde_json::json;

mod complete;
mod cycle;
mod executing;
mod failure;
mod present;
mod scan;
mod selected;
mod sleeping;
mod start;
mod stop;
mod success;
pub(super) struct RunEvents<'a> {
    pub args: &'a ForageArgs,
    pub moonshot_rubric: &'a MoonshotRubric,
    pub bus_handle: &'a BusHandle,
}
