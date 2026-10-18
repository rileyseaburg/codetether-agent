//! Provider/tool iteration and step-budget decisions for a prompt run.
//!
//! [run] connects the shared completion and goal-recovery drivers, then invokes
//! native step preparation, response handling and final persistence via [Effects].
//! [Host] combines those services with the existing driver contracts.
mod budget;
mod completion;
mod effects;
mod run;
pub use effects::Effects;
pub use run::run;
/// Combined services for the iterative prompt lifecycle.
/// # Implementors
/// Hosts implement completion, goal recovery and lifecycle effects.
pub trait Host: Effects + crate::completion::driver::Host + crate::goal_recovery::Host {}
impl<T> Host for T where T: Effects + crate::completion::driver::Host + crate::goal_recovery::Host {}

/// Per-step preparation and thinking notification.
pub mod start;
/// Run persistence, archival and final-answer construction.
pub mod finish;
