//! Execution contracts, tool-request policy and outcome events.
//!
//! [CapabilityLease] limits validity; [requests] handles extraction and
//! validation; [events] formats results. Live tool dispatch belongs to the host.
pub mod events;
mod lease;
mod records;
pub mod requests;
pub use lease::CapabilityLease;
pub use records::{DecisionReceipt, ExecutionOutcome, ToolInvocation};
#[cfg(test)]
mod tests;
