//! Per-session state, in-flight records, and initialization.
//!
//! Runtime owns analysis and delivery state beside the core registry. Request,
//! reply, capture, and event modules implement its behavioral transitions.
mod initialize;
mod pending;
mod state;
pub(crate) use pending::{Pending, QueuedReply};
pub(crate) use state::Runtime;
#[cfg(test)]
mod tests;
