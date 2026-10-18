//! Application identity specialization of the shared LSP client.
mod host;
/// Client advertising the application's version and initialization log event.
pub type LspClient = codetether_lsp_client::LspClient<host::Native>;
/// Server and linter manager for the application-identified client.
pub type LspManager = codetether_lsp_client::LspManager<LspClient>;
