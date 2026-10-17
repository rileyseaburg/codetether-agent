//! Peer introduction classification, payloads and bounded delivery.
//!
//! Detection and canned replies avoid invoking an agent for presence messages.
//! Callers supply identity/token selection and decide when to update a ledger.
pub mod deliver;
pub mod detect;
pub mod payload;
pub mod reply;

/// Introduction-ledger storage over application-selected paths.
pub mod ledger;
