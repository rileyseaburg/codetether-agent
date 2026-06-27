//! Phase 6 — QUIC / WebTransport stream path (endgame).
//!
//! Provides a multiplexed transport over [`quinn`] (QUIC) so that independent
//! logical streams no longer share a single TCP bytestream. A stalled frame on
//! one logical channel cannot block others (no head-of-line blocking), and the
//! connection survives NAT rebinding / IP changes via QUIC connection ids
//! (connection migration).
//!
//! The resume (Phase 1) and bounded-backpressure (Phase 2) models carry over
//! unchanged: QUIC supplies per-stream framing and migration underneath, while
//! event-id cursors and the staging buffer continue to govern replay.
//!
//! See `docs/transport-first-class-plan.md` (Phase 6) for the acceptance
//! experiments: multiplex two streams, stall one, confirm the other keeps
//! delivering; force a path change, confirm migration keeps the session alive.

pub mod client;
pub mod codec;
pub mod migration;
pub mod reader;
pub mod server;
pub mod writer;

pub use client::QuicStreamClient;
pub use reader::QuicFrameReader;
pub use server::QuicStreamServer;
pub use writer::QuicFrameWriter;

/// ALPN protocol identifier negotiated for the A2A QUIC stream path.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::stream::quic::ALPN;
/// assert_eq!(ALPN, b"a2a-stream/1");
/// ```
pub const ALPN: &[u8] = b"a2a-stream/1";
