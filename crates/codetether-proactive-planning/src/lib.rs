//! Bounded planning and latest-generation coalescing for proactive summaries.
//!
//! [planned] schedules multi-scale summary ranges while retaining the active
//! tail. [Slot] keeps only the newest pending snapshot for a worker.
mod plan;
mod ranges;
mod slot;
mod windows;
pub use plan::Plan;
pub use ranges::planned;
pub use slot::Slot;
#[cfg(test)]
mod tests;
