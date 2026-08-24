//! Sandboxed transport construction for an LSP client.

use super::super::{transport::LspTransport, types::LspConfig};
use anyhow::Result;

pub(super) async fn spawn(config: &LspConfig) -> Result<LspTransport> {
    super::super::types::ensure_server_installed(config).await?;
    let workspace = config
        .root_uri
        .as_deref()
        .map(super::super::uri_to_path)
        .unwrap_or(std::env::current_dir()?);
    LspTransport::spawn(&config.command, &config.args, config.timeout_ms, &workspace).await
}
