//! Ordered read-only tool batches with concurrent execution and session recording.
//!
//! [plan] preserves exclusive barriers between contiguous read-only groups.
//! [try_execute] prepares and saves through [Host], runs tools through the
//! execution host, then records [Output] values in their original call order.
mod eligibility;
mod host;
mod job;
mod plan;
mod prepare;
mod record;
mod result;
mod run;
mod single;
pub use host::Host;
pub use plan::{Batch, build as plan};
pub use record::try_execute;
pub use result::Output;
#[cfg(test)]
mod plan_tests;

mod finish;
