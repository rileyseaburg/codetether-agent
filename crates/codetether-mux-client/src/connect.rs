//! Establish an authenticated session or server connection.
use crate::MuxConnection;
use anyhow::{Result, bail};
use codetether_mux_registry::{MuxRecord, SessionTarget};
impl MuxConnection {
    /// Connect for session-scoped operations on `target`.
    /// # Errors
    /// Reports connection, authentication or unsupported-version failures.
    pub async fn connect(target: &SessionTarget) -> Result<Self> {
        Self::open(&target.record, Some(&target.session)).await
    }

    /// Connect for server-scoped operations (session lifecycle, coordination, shutdown).
    /// # Errors
    /// Reports connection, authentication or unsupported-version failures.
    pub async fn connect_server(record: &MuxRecord) -> Result<Self> {
        Self::open(record, None).await
    }

    pub(super) async fn open(record: &MuxRecord, session: Option<&str>) -> Result<Self> {
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
}
