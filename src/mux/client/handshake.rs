//! Version-tolerant authentication used for discovery and shutdown.

use anyhow::{Context, Result, bail};
use tokio::net::TcpStream;

use crate::mux::protocol::{ClientRequest, ServerResponse, read_frame, write_frame};
use crate::mux::registry::MuxRecord;

pub(in crate::mux) async fn connect(record: &MuxRecord) -> Result<(TcpStream, u16)> {
    let mut stream = TcpStream::connect(record.address)
        .await
        .context("connect to mux server")?;
    write_frame(
        &mut stream,
        &ClientRequest::Authenticate {
            token: record.token.clone(),
        },
    )
    .await?;
    match read_frame(&mut stream)
        .await?
        .context("mux server closed during authentication")?
    {
        ServerResponse::Authenticated { version } => Ok((stream, version)),
        ServerResponse::Error { message } => bail!(message),
        _ => bail!("mux server returned an invalid authentication response"),
    }
}

pub(in crate::mux) async fn probe(record: &MuxRecord) -> Result<u16> {
    connect(record).await.map(|(_, version)| version)
}
