//! Version-tolerant authentication used for discovery and shutdown.

use anyhow::{Context, Result, bail};
use tokio::net::TcpStream;

use codetether_mux_protocol::{ClientRequest, ServerResponse, read_frame, write_frame};
use codetether_mux_registry::MuxRecord;

/// Authenticate against a server, optionally binding the connection to a session.
/// # Arguments
/// `record` supplies endpoint and credential; `session` optionally selects a session.
/// # Returns
/// The authenticated stream and reported protocol version.
/// # Errors
/// Reports connection/framing failures or a server authentication error.
/// # Examples
/// ```no_run
/// # async fn example(record: &codetether_mux_registry::MuxRecord) -> anyhow::Result<()> {
/// let (_stream, _version) = codetether_mux_client::handshake::connect(record, None).await?;
/// # Ok(()) }
/// ```
pub async fn connect(record: &MuxRecord, session: Option<&str>) -> Result<(TcpStream, u16)> {
    let mut stream = TcpStream::connect(record.address)
        .await
        .context("connect to mux server")?;
    write_frame(
        &mut stream,
        &ClientRequest::Authenticate {
            token: record.token.clone(),
            session: session.map(str::to_string),
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

/// Server-scoped liveness probe.
/// # Arguments
/// `record` supplies the endpoint and credential to probe.
/// # Returns
/// The reported version without imposing the client's compatibility check.
/// # Errors
/// Propagates network, framing and authentication failures.
/// # Examples
/// ```no_run
/// # async fn example(record: &codetether_mux_registry::MuxRecord) -> anyhow::Result<()> {
/// let _version = codetether_mux_client::handshake::probe(record).await?;
/// # Ok(()) }
/// ```
pub async fn probe(record: &MuxRecord) -> Result<u16> {
    connect(record, None).await.map(|(_, version)| version)
}
