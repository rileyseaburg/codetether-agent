//! Resumable A2A stream protocol (Phase 1).
//!
//! Implements transport-agnostic stream resumption over SSE: monotonic event
//! ids, a durable client cursor, event classification, `Last-Event-ID` reconnect
//! injection, and `resync-required` handling. See
//! `docs/transport-phase1-wire-contract.md` for the wire contract.

//!
//! Use [event_id::EventId] and [frame::ParsedFrame] for the wire format,
//! [cursor::Cursor] for persisted resume progress, and [quic] for multiplexed
//! transport. [backoff], [breaker] and [lifecycle] supply reconnect policy.
//!
//! ```rust
//! use codetether_a2a_stream::frame::parse_frame;
//! let frame = parse_frame("id: epoch.7\nevent: progress\ndata: ready").unwrap();
//! assert_eq!(frame.id.unwrap().seq, 7);
//! assert_eq!(frame.data, "ready");
//! ```
pub mod backoff;
pub mod breaker;
pub mod classify;
pub mod cursor;
#[cfg(test)]
mod cursor_tests;
pub mod event_id;
pub mod frame;
pub mod lifecycle;
pub mod probe;
pub mod quic;
pub mod resume_request;
pub mod resync;
pub mod staging;

/// Bounded frame ingestion and caller-side frame handling.
pub mod ingest;
/// Reconnect outcome policy and backoff progression.
pub mod reconnect;

/// Resumable worker SSE request construction.
pub mod worker_request;

/// Worker event dispatch and resume-cursor progression.
pub mod dispatch;
