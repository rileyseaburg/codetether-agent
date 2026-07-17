//! Parallel read-only tool execution for TUI turns.

mod eligibility;
mod job;
mod plan;
#[cfg(test)]
mod plan_tests;
mod record;
mod result;
mod route;
mod run;
mod single;

pub(in crate::session::helper) use plan::{Batch, build as plan};
pub(in crate::session::helper) use record::try_execute;
