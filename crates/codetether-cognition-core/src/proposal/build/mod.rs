//! Proposal and proposal-created event construction.
//!
//! [build_proposal] freezes quorum and deadline values from governance.
//! [created_event] publishes the resulting record with thought provenance.
mod event;
mod proposal;
pub use event::created_event;
pub use proposal::build_proposal;
