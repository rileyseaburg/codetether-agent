//! Worker-side QUIC dial path for the A2A stream transport.
//!
//! Connects to a server [`QuicListener`](super::listener::QuicListener), opens a
//! bidirectional stream, and exposes a [`QuicFrameReader`] seeded with the
//! durable resume cursor's last seq. Gated behind `quic-transport`; this is the
//! opt-in replacement for the reqwest/SSE dial in `task_stream.rs`.

use super::client::QuicStreamClient;
use super::reader::QuicFrameReader;
use crate::a2a::stream::cursor::Cursor;
use anyhow::{Context, Result};
use quinn::Connection;
use rustls::pki_types::CertificateDer;
use std::net::SocketAddr;

/// An established worker-side QUIC stream session with resume applied.
pub struct QuicStreamSession {
    /// The live QUIC connection (retain to keep streams open / enable migration).
    pub connection: Connection,
    /// Frame reader seeded with the resume floor from the durable cursor.
    pub reader: QuicFrameReader,
}

/// Dial `addr`, open a bidi stream, and build a resume-aware frame reader.
///
/// The resume floor is taken from `cursor`'s last processed seq, so a fresh
/// connection after a drop or migration skips already-handled events.
///
/// # Errors
/// Returns an error if the handshake, connect, or stream open fails.
pub async fn dial_session(
    client: &QuicStreamClient,
    addr: SocketAddr,
    server_name: &str,
    cursor: &Cursor,
) -> Result<QuicStreamSession> {
    let connection = client.connect(addr, server_name).await?;
    let (mut send, recv) = connection.open_bi().await.context("open bidi stream")?;
    // Nudge the stream open so the server's accept_bi resolves.
    send.write_all(&0u32.to_be_bytes())
        .await
        .context("prime quic stream")?;
    let resume_seq = cursor.last().map(|id| id.seq);
    Ok(QuicStreamSession {
        connection,
        reader: QuicFrameReader::new(recv, resume_seq),
    })
}

/// Re-trust helper: build a client from in-memory roots (e.g. pinned server cert).
///
/// # Errors
/// Returns an error if the trust store rejects a certificate.
pub fn client_with_roots(roots: Vec<CertificateDer<'static>>) -> Result<QuicStreamClient> {
    QuicStreamClient::with_roots(roots)
}
