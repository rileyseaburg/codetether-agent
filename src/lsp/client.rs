//! Application identity adapter and multi-server LSP manager.

use super::types::*;
use anyhow::Result;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// LSP client that preserves application initialization identity.
pub struct LspClient {
    inner: codetether_lsp_core::client::LspClient,
}

/// LSP Manager - manages multiple language server connections
pub struct LspManager {
    pub(super) clients: super::client_cache::ClientCache,
    /// Linter clients keyed by linter name (e.g. "eslint", "ruff").
    /// These are only queried for diagnostics, not completions/definitions.
    linter_clients: super::client_cache::ClientCache,
    root_uri: Option<String>,
    /// User-supplied LSP settings from config.
    lsp_settings: Option<crate::config::LspSettings>,
}

mod deref;
mod initialize;
mod lifecycle;
mod linter_config;
mod linter_diagnostics;
mod manager_capabilities;
mod manager_connections;
mod manager_default;
mod manager_documents;
mod manager_lifecycle;
mod manager_linters;
