//! Peer introduction classification, payloads and bounded delivery.
//!
//! Detection and canned replies avoid invoking an agent for presence messages.
//! [send::send_intro] owns deduplication and token selection; callers supply
//! a ledger-path resolver so application configuration stays outside this crate.
pub mod deliver;
pub mod detect;
pub mod payload;
pub mod reply;

/// Introduction-ledger storage over application-selected paths.
pub mod ledger;

mod identity;
/// Deduplicated introduction delivery over a host-selected ledger.
pub mod send;
