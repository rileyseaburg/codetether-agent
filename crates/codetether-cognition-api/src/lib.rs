//! Cognition HTTP projection, query ordering and event transport.
//!
//! Callers capture runtime state and pass records into these response functions.
//! This crate owns HTTP representation without depending on the live runtime.
mod beliefs;
mod filter;
mod proposals;
mod records;
mod stream;
pub use beliefs::list_beliefs;
pub use filter::BeliefFilter;
pub use proposals::list_proposals;
pub use records::{belief, latest_snapshot, persona};
pub use stream::stream_cognition;
