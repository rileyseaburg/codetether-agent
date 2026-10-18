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

pub use client::LspManager;
pub use codetether_lsp_core::{tetherscript, transport, types, uri, workspace_folder};
pub use types::*;
pub use uri::{path_to_uri, uri_to_path};
