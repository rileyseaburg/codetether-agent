//! Reuse authenticated bindings and exchange framed requests.
use crate::MuxConnection;
use anyhow::{Context, Result};
use codetether_mux_protocol::{ClientRequest, ServerResponse, read_frame, write_frame};
impl MuxConnection {
    /// Open another authenticated connection with the same session binding.
    /// # Errors
    /// Reports connection, authentication or unsupported-version failures.
    pub async fn secondary(&self) -> Result<Self> {
        Self::open(&self.record, self.session.as_deref()).await
    }

    /// Return the authenticated server's protocol version.
    pub fn version(&self) -> u16 {
        self.version
    }

    /// Send a framed request and read its response on this connection.
    /// # Errors
    /// Reports framing, network, decoding or unexpected EOF failures.
    pub async fn request(&mut self, request: ClientRequest) -> Result<ServerResponse> {
        write_frame(&mut self.stream, &request).await?;
        read_frame(&mut self.stream)
            .await?
            .context("mux server closed the connection")
    }
}
