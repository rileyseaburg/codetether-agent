//! Worker dispatch targeting, concurrency slots and task execution budgets.
//!
//! [explicitly_targets_agent] applies envelope targeting rules.
//! [reserve_task_slot] reports [TaskReservation] and [execute_with_timeout]
//! applies a per-task deadline without depending on the application runtime.
mod reservation;
mod slot;
mod targeting;
mod timeout;
pub use reservation::TaskReservation;
pub use slot::reserve_task_slot;
pub use targeting::{explicitly_targets_agent, targeted_only_enabled};
pub use timeout::execute_with_timeout;
#[cfg(test)]
mod targeting_tests;
#[cfg(test)]
mod timeout_tests;

/// Ordered startup and periodic pending-task dispatch.
pub mod pending;
