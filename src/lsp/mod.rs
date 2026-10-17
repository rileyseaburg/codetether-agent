//! LSP (Language Server Protocol) client implementation
//!
//! Provides a complete LSP client for code intelligence features:
//! - Go to definition
//! - Find references
//! - Hover information
//! - Document symbols
//! - Workspace symbols
//! - Code completion
//!
//! Supports language servers via stdio transport with JSON-RPC 2.0.

pub mod client;
mod client_cache;
mod client_recovery;
#[cfg(test)]
mod typescript_repeated_tests;

pub use client::LspManager;
pub use codetether_lsp_core::{tetherscript, transport, types, uri, workspace_folder};
pub use types::*;
pub use uri::{path_to_uri, uri_to_path};

#[cfg(test)]
#[path = "tetherscript_smoke_tests.rs"]
mod tetherscript_smoke_tests;

#[cfg(test)]
mod jsx_diagnostics_tests;
#[cfg(test)]
#[path = "proposed_diagnostics_tests.rs"]
mod proposed_diagnostics_tests;
