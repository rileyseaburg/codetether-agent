//! Initialize an LSP transport using caller-supplied client identity.
//!
//! [run] sends the protocol handshake, stores server capabilities, then marks
//! the transport initialized. The caller supplies version identity and logging.
mod params;
use crate::{InitializeResult, LspConfig, ServerInfo, transport::LspTransport};
use anyhow::Result;
use lsp_types::ServerCapabilities;
use tokio::sync::RwLock;

/// Perform the initialize/initialized handshake for an existing transport.
///
/// # Arguments
/// * `transport` — Connected language-server transport.
/// * `config` — Root and initialization settings.
/// * `capabilities` — Destination for the returned server capabilities.
/// * `client_version` — Application version advertised in clientInfo.
/// * `on_initialized` — Observer called after capabilities are stored.
/// # Returns
/// Unit once the initialized notification was sent and state updated.
/// # Errors
/// Returns errors from serialization, transport, or an initialize error reply.
/// # Examples
/// ```no_run
/// # async fn example(transport: &codetether_lsp_core::transport::LspTransport, config: &codetether_lsp_core::LspConfig) -> anyhow::Result<()> {
/// let capabilities = tokio::sync::RwLock::new(None);
/// codetether_lsp_core::initialize::run(transport, config, &capabilities, "1.0", |_| {}).await?;
/// # Ok(()) }
/// ```
pub async fn run(
    transport: &LspTransport,
    config: &LspConfig,
    capabilities: &RwLock<Option<ServerCapabilities>>,
    client_version: &str,
    on_initialized: impl FnOnce(&Option<ServerInfo>),
) -> Result<()> {
    let params = params::build(config, client_version);
    let response = transport
        .request("initialize", Some(serde_json::to_value(params)?))
        .await?;
    if let Some(error) = response.error {
        return Err(anyhow::anyhow!("LSP initialize error: {}", error.message));
    }
    if let Some(result) = response.result {
        let init_result: InitializeResult = serde_json::from_value(result)?;
        *capabilities.write().await = Some(init_result.capabilities);
        on_initialized(&init_result.server_info);
    }
    transport.notify("initialized", None).await?;
    transport.set_initialized(true);
    if let Some(root_uri) = config.root_uri.as_deref() {
        transport.start_diagnostic_sharing(root_uri);
    }
    Ok(())
}
