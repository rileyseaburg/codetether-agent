//! Resumable A2A stream protocol (Phase 1).
//!
//! Implements transport-agnostic stream resumption over SSE: monotonic event
//! ids, a durable client cursor, event classification, `Last-Event-ID` reconnect
//! injection, and `resync-required` handling. See
//! `docs/transport-phase1-wire-contract.md` for the wire contract.

pub mod backoff;
pub mod breaker;
pub mod classify;
pub mod cursor;
#[cfg(test)]
mod cursor_tests;
pub mod event_id;
pub mod frame;
pub mod lifecycle;
pub mod resume_request;
pub mod resync;
pub mod socket_opts;
pub mod staging;
pub mod tcp_info;
