//! Proactive model rate gating using a rolling 60-second usage window.
//!
//! [check_rate_gate] chooses an available substitute when the primary is at
//! its configured limit. [record_usage] accounts completed requests. The
//! process-wide [RATE_GATE] and standalone [RateGate] share the same policy.
mod gate;
mod limits;
mod routing;
mod shared;
mod window;
pub use gate::RateGate;
pub use routing::{check_rate_gate, record_usage};
pub use shared::RATE_GATE;
#[cfg(test)]
mod tests;
