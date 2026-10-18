//! Application version and log event retained at the native boundary.
/// Application identity used by the public LSP client and manager aliases.
pub struct Native;
impl codetether_lsp_client::Host for Native {
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn initialized(server_info: &Option<codetether_lsp_core::types::ServerInfo>) {
        tracing::info!(target: "codetether_agent::lsp::client::initialize",
            server_info = ?server_info, "LSP server initialized");
    }
}
