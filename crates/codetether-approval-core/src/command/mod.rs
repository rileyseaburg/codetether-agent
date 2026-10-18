//! Parsing and persistence of explicit approval command decisions.
//!
//! Parse an Action, resolve its optional request ID in the host, then record
//! the decision in an ApprovalStore. Live delivery and UI queues remain host-owned.
mod intent;
mod message;
mod parse;
mod store;
pub use intent::ApprovalIntent;
pub use message::text as decision_text;
pub use parse::Action;
pub use store::{StoredDecision, record};
