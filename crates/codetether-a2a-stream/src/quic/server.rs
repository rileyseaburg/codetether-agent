//! QUIC server endpoint for the multiplexed A2A stream path.

use super::ALPN;
use anyhow::{Context, Result};
use quinn::{Endpoint, Incoming, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::net::SocketAddr;
use std::sync::Arc;

/// A bound QUIC server endpoint that accepts multiplexed stream connections.
///
/// Each accepted [`quinn::Connection`] can carry many independent bidirectional
/// streams; a stall on one does not block the others (no head-of-line blocking).
pub struct QuicStreamServer {
    endpoint: Endpoint,
}

impl QuicStreamServer {
    /// Bind a server endpoint on `addr` presenting `cert_chain` / `key`.
    ///
    /// # Errors
    /// Returns an error if the TLS config is rejected or the UDP socket cannot
    /// be bound.
    pub fn bind(
        addr: SocketAddr,
        cert_chain: Vec<CertificateDer<'static>>,
        key: PrivateKeyDer<'static>,
    ) -> Result<Self> {
        let mut crypto = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)
            .context("build rustls server config")?;
        crypto.alpn_protocols = vec![ALPN.to_vec()];
        let qsc = quinn::crypto::rustls::QuicServerConfig::try_from(crypto)
            .context("rustls -> quic server config")?;
        let endpoint = Endpoint::server(ServerConfig::with_crypto(Arc::new(qsc)), addr)
            .context("bind quic server endpoint")?;
        Ok(Self { endpoint })
    }

    /// Local socket address the endpoint is bound to.
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.endpoint.local_addr().context("local_addr")
    }

    /// Await the next inbound connection attempt.
    pub async fn accept(&self) -> Option<Incoming> {
        self.endpoint.accept().await
    }
}
