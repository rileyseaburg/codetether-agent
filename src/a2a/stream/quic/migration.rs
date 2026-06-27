//! Connection-migration support for the QUIC client.
//!
//! Rebinding the endpoint to a fresh local UDP socket changes the client's
//! source address. Because QUIC identifies connections by connection id rather
//! than the 4-tuple, an in-flight session survives the path change (no cold
//! resync) — this is the migration half of the Phase 6 acceptance experiment.

use super::client::QuicStreamClient;
use anyhow::{Context, Result};

impl QuicStreamClient {
    /// Rebind the endpoint to a new ephemeral local UDP port, forcing a path
    /// change that exercises QUIC connection migration on live connections.
    ///
    /// # Errors
    /// Returns an error if a fresh UDP socket cannot be bound or the endpoint
    /// rejects the rebind.
    pub fn migrate_local_port(&self) -> Result<()> {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0").context("bind migration socket")?;
        self.endpoint().rebind(socket).context("rebind endpoint")
    }
}
