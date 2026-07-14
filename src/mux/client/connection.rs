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
        let stream = TcpStream::connect(record.address)
            .await
            .context("connect to mux server")?;
        let mut connection = Self { stream };
        let response = connection
            .request(ClientRequest::Authenticate {
                token: record.token.clone(),
            })
            .await?;
        match response {
            ServerResponse::Authenticated { version: VERSION } => Ok(connection),
            ServerResponse::Authenticated { version } => {
                bail!("unsupported mux protocol version {version}")
            }
            ServerResponse::Error { message } => bail!("{message}"),
            _ => bail!("mux server returned an invalid authentication response"),
        }
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
