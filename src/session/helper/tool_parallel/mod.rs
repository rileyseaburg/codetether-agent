//! Parallel read-only tool execution for TUI turns.

mod eligibility;
mod job;
mod record;
mod result;
mod route;
mod run;
mod single;

pub(super) use record::try_execute;
