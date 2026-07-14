//! Hidden mux server process entry point.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Result;

pub(super) async fn run(session: String, directory: PathBuf, bind: SocketAddr) -> Result<()> {
    crate::mux::registry::validate_name(&session)?;
    let directory = tokio::fs::canonicalize(directory).await?;
    crate::mux::server::serve(session, directory, bind).await
}
