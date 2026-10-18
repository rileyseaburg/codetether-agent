//! SSE conversion of general bus envelopes and task-specific output.
//!
//! bus_events filters by caller-supplied topic claims. task_events accepts only
//! exact task topics and payload IDs, reporting a terminal gap after lag.
mod bus;
mod filter;
mod service;
mod topic;
pub use bus::events as bus_events;
pub use service::events as task_events;
pub use topic::topic_matches;
#[cfg(test)]
mod tests;
