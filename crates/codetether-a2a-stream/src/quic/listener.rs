//! Server-side QUIC listener loop for the A2A worker stream transport.
//!
//! Wraps [`QuicStreamServer`] with an accept loop that hands each inbound
//! connection to a handler. Gated behind the `quic-transport` feature: the QUIC
//! primitives always compile, but the live server wiring is opt-in.

use super::server::QuicStreamServer;
use anyhow::{Context, Result};
use quinn::Connection;

/// Accept loop over a bound [`QuicStreamServer`].
pub struct QuicListener {
    server: QuicStreamServer,
}

impl QuicListener {
    /// Wrap a bound server endpoint in an accept loop.
    pub fn new(server: QuicStreamServer) -> Self {
        Self { server }
    }

    /// Await and fully establish the next inbound connection.
    ///
    /// # Errors
    /// Returns an error if the endpoint is closed or the handshake fails.
    pub async fn accept_connection(&self) -> Result<Option<Connection>> {
        let Some(incoming) = self.server.accept().await else {
            return Ok(None);
        };
        let conn = incoming
            .accept()
            .context("accept quic incoming")?
            .await
            .context("establish quic connection")?;
        Ok(Some(conn))
    }
}
