//! Resumable A2A stream protocol (Phase 1).
//!
//! Implements transport-agnostic stream resumption over SSE: monotonic event
//! ids, a durable client cursor, event classification, `Last-Event-ID` reconnect
//! injection, and `resync-required` handling. See
//! `docs/transport-phase1-wire-contract.md` for the wire contract.

pub mod classify;
pub mod cursor;
pub mod event_id;
pub mod frame;
pub mod resume_request;
pub mod resync;
