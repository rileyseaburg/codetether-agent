//! Dependency-free cooperative async scheduling primitives.
//!
//! These types track task readiness, completed results, and join waiters
//! without pulling in Tokio or any external event loop.

mod finish;
mod join;
mod queue;
mod spawn;
mod task;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
pub use queue::Scheduler;
#[allow(unused_imports)]
pub use task::{Task, TaskState};
