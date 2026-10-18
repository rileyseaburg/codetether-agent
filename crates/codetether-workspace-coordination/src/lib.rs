//! Fail-closed mutation gating against an authoritative workspace coordinator.
//!
//! [blocked] checks coordinator activity before extracting mutation paths and
//! trusted runtime identity. [blocked_in_mux] handles already-active contexts.
//! Both return structured blocking results without executing the proposed tool.
mod context;
mod gate;
mod gate_error;
mod gate_failure;
mod host;
mod outcome;
mod scope_error;
use codetether_mutation_scope::{paths, scope};
pub use gate::{blocked, blocked_in_mux};
pub use host::Host;
#[cfg(test)]
mod scope_tests;
#[cfg(test)]
mod tool_tests;

mod lease_host;
mod release;
mod renewal;
mod turn;
mod turn_close;
mod turn_drop;
mod turn_open;
pub use lease_host::LeaseHost;
pub use turn::LeaseTurn;
#[cfg(test)]
mod test_lease_host;
#[cfg(test)]
mod turn_tests;
