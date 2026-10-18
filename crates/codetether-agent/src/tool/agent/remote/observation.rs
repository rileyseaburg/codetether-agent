//! Shared peer observations with native TUI transcript integration coverage.
pub(in crate::tool::agent) use codetether_peer_agent::observation::{
    begin, live_trace, snapshots, transcript, types,
};
#[cfg(test)]
#[path = "observation_reply_tests.rs"]
mod reply_tests;
