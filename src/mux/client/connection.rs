//! Authenticated request/response mux connection.

use anyhow::{Context, Result, bail};
use tokio::net::TcpStream;

use crate::mux::protocol::{ClientRequest, ServerResponse, VERSION, read_frame, write_frame};
use crate::mux::registry::MuxRecord;

pub(in crate::mux) struct MuxConnection {
    stream: TcpStream,
    record: MuxRecord,
    version: u16,
}

impl MuxConnection {
    pub(in crate::mux) async fn connect(record: &MuxRecord) -> Result<Self> {
        let (stream, version) = super::handshake::connect(record).await?;
        if !supported_version(version) {
            bail!("unsupported mux protocol version {version}");
        }
        Ok(Self {
            stream,
            record: record.clone(),
            version,
        })
    }

    pub(super) async fn secondary(&self) -> Result<Self> {
        Self::connect(&self.record).await
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

fn supported_version(version: u16) -> bool {
    (6..=VERSION).contains(&version)
}

#[cfg(test)]
#[path = "connection_tests.rs"]
mod tests;
