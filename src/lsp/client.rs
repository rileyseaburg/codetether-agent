//! Application initialization identity for shared LSP clients and managers.

use super::types::*;
use anyhow::Result;

/// LSP client that preserves application initialization identity.
pub struct LspClient {
    inner: codetether_lsp_core::client::LspClient,
}

/// Shared manager specialized for the application identity wrapper.
pub type LspManager = codetether_lsp_core::manager::LspManager<LspClient>;

mod deref;
mod initialize;
mod lifecycle;
mod managed;
