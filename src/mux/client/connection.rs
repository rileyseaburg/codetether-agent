//! Authenticated request/response mux connection bound to one session.

use anyhow::{Context, Result, bail};
use tokio::net::TcpStream;

use crate::mux::protocol::{ClientRequest, ServerResponse, read_frame, write_frame};
use crate::mux::registry::{MuxRecord, SessionTarget};

pub(in crate::mux) struct MuxConnection {
    stream: TcpStream,
    record: MuxRecord,
    session: Option<String>,
    version: u16,
}

impl MuxConnection {
    /// Connect for session-scoped operations on `target`.
    pub(in crate::mux) async fn connect(target: &SessionTarget) -> Result<Self> {
        Self::open(&target.record, Some(&target.session)).await
    }

    /// Connect for server-scoped operations (session lifecycle, coordination, shutdown).
    pub(in crate::mux) async fn connect_server(record: &MuxRecord) -> Result<Self> {
        Self::open(record, None).await
    }

    async fn open(record: &MuxRecord, session: Option<&str>) -> Result<Self> {
        let (stream, version) = super::handshake::connect(record, session).await?;
        if !super::connection_version::supported(version) {
            bail!("unsupported mux protocol version {version}");
        }
        Ok(Self {
            stream,
            record: record.clone(),
            session: session.map(str::to_string),
            version,
        })
    }

    pub(super) async fn secondary(&self) -> Result<Self> {
        Self::open(&self.record, self.session.as_deref()).await
    }

    pub(in crate::mux) fn version(&self) -> u16 {
        self.version
    }

    pub(in crate::mux) async fn request(
        &mut self,
        request: ClientRequest,
    ) -> Result<ServerResponse> {
        write_frame(&mut self.stream, &request).await?;
        read_frame(&mut self.stream)
            .await?
            .context("mux server closed the connection")
    }
}
