//! Stream result assembly and final acceptance without transport ownership.
//!
//! [build_response] preserves thinking, text and tool-call order. [StreamStop]
//! records why a drain ended; [accept] rejects incomplete transport results.
//! [fault_from] classifies provider error messages for restart decisions.
mod accept;
mod fault;
mod finalize;
mod idle_fault;
mod limits;
mod markers;
mod outcome;
mod stop;
mod tool;
pub use accept::{accept, exhausted};
pub use fault::{display_message, is_transient};
pub use finalize::build_response;
pub use idle_fault::fault_from;
pub use limits::IDLE_TIMEOUT;
pub use outcome::DrainOutcome;
pub use stop::StreamStop;
pub use tool::ToolAccumulator;
#[cfg(test)]
mod tests;
