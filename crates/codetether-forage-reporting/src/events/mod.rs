//! Borrowed adapters for forage console, bus and audit reporting.
//!
//! [RunEvents] references the run's existing arguments, rubric and bus handle.
//! Its methods report lifecycle events; execution and loop state stay with callers.
use super::*;
use codetether_a2a_core::{Part, TaskState};
use codetether_audit::{AuditCategory, AuditOutcome};
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
/// Borrowed reporting context for a single forage run.
///
/// # Examples
/// ```no_run
/// use codetether_forage_reporting::RunEvents;
/// fn events<'a>(args: &'a codetether_cli_options::ForageArgs,
///     rubric: &'a codetether_forage_core::MoonshotRubric,
///     bus: &'a codetether_bus_core::BusHandle) -> RunEvents<'a> {
///     RunEvents { args, moonshot_rubric: rubric, bus_handle: bus }
/// }
/// ```
pub struct RunEvents<'a> {
    /// Run options used to label events and select output mode.
    pub args: &'a ForageArgs,
    /// Rubric included in startup and selected-opportunity reports.
    pub moonshot_rubric: &'a MoonshotRubric,
    /// Existing run bus identity used for all messages.
    pub bus_handle: &'a BusHandle,
}
