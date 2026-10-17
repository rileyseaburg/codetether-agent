//! LSP Client - manages language server connections and operations
//!
//! Provides high-level API for LSP operations:
//! - Initialize/shutdown lifecycle
//! - Document synchronization
//! - Code intelligence (definition, references, hover, etc.)

use super::transport::LspTransport;

use super::types::*;

use super::uri::path_to_uri;

use anyhow::Result;

use lsp_types::{
    CompletionContext, CompletionParams, CompletionTriggerKind, DocumentSymbolParams, HoverParams,
    Position, TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams,
};

use std::collections::HashMap;

use std::path::Path;

use std::sync::Arc;

use tokio::sync::RwLock;

use tracing::{debug, info, warn};

/// LSP Client for a single language server
pub struct LspClient {
    pub(super) last_used: std::sync::Mutex<std::time::Instant>,
    pub(super) transport: LspTransport,
    pub(super) config: LspConfig,
    pub(super) server_capabilities: RwLock<Option<lsp_types::ServerCapabilities>>,
    /// Track open documents with their versions
    pub(super) open_documents: RwLock<HashMap<String, i32>>,
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

mod capabilities;
mod completion;
mod document_changes;
mod documents;
mod hover;
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
mod navigation;
mod references;
mod symbols;
use codetether_lsp_core::response::parse_completion_response;
use codetether_lsp_core::response::parse_document_symbols_response;
use codetether_lsp_core::response::parse_hover_response;
use codetether_lsp_core::response::parse_location_response;
use codetether_lsp_core::uri::parse_uri;
use codetether_lsp_core::response::parse_workspace_symbols_response;
