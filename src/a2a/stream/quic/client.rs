//! QUIC client endpoint for the multiplexed A2A stream path.

use super::ALPN;
use anyhow::{Context, Result};
use quinn::{ClientConfig, Connection, Endpoint};
use rustls::pki_types::CertificateDer;
use std::net::SocketAddr;
use std::sync::Arc;

/// A QUIC client endpoint that opens multiplexed connections to a server.
pub struct QuicStreamClient {
    endpoint: Endpoint,
}

impl QuicStreamClient {
    /// Build a client that trusts exactly the supplied `roots` (e.g. a
    /// self-signed loopback cert for tests).
    ///
    /// # Errors
    /// Returns an error if the unspecified-port UDP socket cannot be bound or
    /// the trust store rejects a certificate.
    pub fn with_roots(roots: Vec<CertificateDer<'static>>) -> Result<Self> {
        let mut store = rustls::RootCertStore::empty();
        for cert in roots {
            store.add(cert).context("add root cert")?;
        }
        let mut crypto = rustls::ClientConfig::builder()
            .with_root_certificates(store)
            .with_no_client_auth();
        crypto.alpn_protocols = vec![ALPN.to_vec()];
        let qcc = quinn::crypto::rustls::QuicClientConfig::try_from(crypto)
            .context("rustls -> quic client config")?;
        let mut endpoint = Endpoint::client("0.0.0.0:0".parse().context("client addr")?)
            .context("bind quic client endpoint")?;
        endpoint.set_default_client_config(ClientConfig::new(Arc::new(qcc)));
        Ok(Self { endpoint })
    }

    /// Connect to `addr`, validating the server certificate against `server_name`.
    ///
    /// # Errors
    /// Returns an error if the handshake fails or is rejected.
    pub async fn connect(&self, addr: SocketAddr, server_name: &str) -> Result<Connection> {
        self.endpoint
            .connect(addr, server_name)
            .context("start quic connect")?
            .await
            .context("complete quic handshake")
    }
}
