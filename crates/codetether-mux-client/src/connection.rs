//! Authenticated connection state, including its session binding.
use codetether_mux_registry::MuxRecord;
use tokio::net::TcpStream;

/// Authenticated TCP connection retaining its server record and session binding.
/// # Examples
/// ```no_run
/// # async fn example(target: &codetether_mux_registry::SessionTarget) -> anyhow::Result<()> {
/// let connection = codetether_mux_client::MuxConnection::connect(target).await?;
/// assert_eq!(connection.version(), codetether_mux_protocol::VERSION);
/// # Ok(()) }
/// ```
pub struct MuxConnection {
    pub(super) stream: TcpStream,
    pub(super) record: MuxRecord,
    pub(super) session: Option<String>,
    pub(super) version: u16,
}
