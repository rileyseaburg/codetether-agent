//! Native session snapshot binding for crate-owned live event capture.
pub(super) type LiveEvents = codetether_run_events::LiveEvents<crate::session::Session>;
