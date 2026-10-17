//! Protocol data, server configuration, and stdio transport for LSP clients.
//!
//! Use [`LspConfig`] to select a server and [`transport::LspTransport`] to
//! exchange framed JSON-RPC messages. Applications own initialization identity,
//! connection caches, and user-facing policy.
//!
//! ```
//! use codetether_lsp_core::get_language_server_config;
//! let config = get_language_server_config("rust").unwrap();
//! assert!(config.file_extensions.iter().any(|extension| extension == "rs"));
//! ```

pub mod response;
pub mod tetherscript;
pub mod transport;
pub mod types;
pub mod uri;
pub mod workspace_folder;
pub use types::*;
pub use uri::{path_to_uri, uri_to_path};
#[cfg(test)]
mod tetherscript_config_tests;
#[cfg(test)]
mod tetherscript_tests;
#[cfg(test)]
mod workspace_folder_tests;

/// Client capability declaration for diagnostic-producing servers.
pub mod client_capabilities;
/// Wait for a document-specific diagnostic publication.
pub mod client_diagnostics_wait;
/// Protocol language IDs, including JSX and TSX dialects.
pub mod document_language;
/// Decode TypeScript synchronous diagnostic payloads.
pub mod typescript_diagnostic;

/// LSP handshake with explicit client identity and capability storage.
pub mod initialize;
