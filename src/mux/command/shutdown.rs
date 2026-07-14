//! Backward-compatible shutdown request for current and legacy mux servers.

use anyhow::{Context, Result, bail};

use crate::mux::protocol::{ClientRequest, ServerResponse, VERSION, read_frame, write_frame};
use crate::mux::registry::MuxRecord;

pub(super) async fn request(record: &MuxRecord) -> Result<ServerResponse> {
    let (mut stream, version) = crate::mux::client::handshake::connect(record).await?;
    if !(1..=VERSION).contains(&version) {
        bail!("unsupported mux protocol version {version}");
    }
    write_frame(&mut stream, &ClientRequest::Shutdown).await?;
    read_frame(&mut stream)
        .await?
        .context("mux server closed before confirming shutdown")
}
