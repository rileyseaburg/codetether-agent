//! Backend-independent run result and dependency-failure policy.
//!
//! [partition] prevents dependent work from running after a prerequisite failure.
//! [finalize_stats] checks completion and computes run statistics; [aggregate_results]
//! renders per-task output. [empty_result] describes a decomposition with no tasks.
mod aggregate;
mod dependency_failure;
mod empty;
mod stats;
pub use aggregate::aggregate_results;
pub use dependency_failure::partition;
pub use empty::empty_result;
pub use stats::finalize_stats;

mod expected_changes;
pub use expected_changes::for_tasks as expected_changes;
mod decomposed;
pub use decomposed::build as decomposed_event;
mod failure;
pub use failure::result as failure_result;
mod blocked_events;
pub use blocked_events::report as report_blocked;
