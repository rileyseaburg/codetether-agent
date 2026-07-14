//! Authenticated request/response mux connection.

use anyhow::{Context, Result, bail};
use tokio::net::TcpStream;

use crate::mux::protocol::{ClientRequest, ServerResponse, VERSION, read_frame, write_frame};
use crate::mux::registry::MuxRecord;

pub(in crate::mux) struct MuxConnection {
    stream: TcpStream,
}

impl MuxConnection {
    pub(in crate::mux) async fn connect(record: &MuxRecord) -> Result<Self> {
        let (stream, version) = super::handshake::connect(record).await?;
        if version != VERSION {
            bail!("unsupported mux protocol version {version}");
        }
        Ok(Self { stream })
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
