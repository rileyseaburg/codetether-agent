//! Single-server LSP state, document synchronization and query operations.
//!
//! Construct [LspClient], initialize with caller identity, then issue document
//! and code-intelligence requests. Managers own cache and routing policy.

use crate::transport::LspTransport;

use crate::types::*;

use crate::uri::path_to_uri;

use anyhow::Result;

use lsp_types::{
    CompletionContext, CompletionParams, CompletionTriggerKind, DocumentSymbolParams, HoverParams,
    Position, TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams,
};

use std::collections::HashMap;

use std::path::Path;

use tokio::sync::RwLock;

use tracing::{debug, info, warn};

/// LSP client that owns transport, document versions and server capabilities.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let client = codetether_lsp_core::LspClient::for_language("rust", None).await?;
/// client.initialize_with_identity("1.0", |_| {}).await?;
/// client.shutdown().await?;
/// # Ok(()) }
/// ```
pub struct LspClient {
    pub(super) last_used: std::sync::Mutex<std::time::Instant>,
    pub(super) transport: LspTransport,
    pub(super) config: LspConfig,
    pub(super) server_capabilities: RwLock<Option<lsp_types::ServerCapabilities>>,
    /// Track open documents with their versions
    pub(super) open_documents: RwLock<HashMap<String, i32>>,
}

mod access;
mod capabilities;
mod completion;
mod document_changes;
mod documents;
mod hover;
mod initialize;
mod lifecycle;
mod navigation;
mod references;
mod symbols;
use crate::response::parse_completion_response;
use crate::response::parse_document_symbols_response;
use crate::response::parse_hover_response;
use crate::response::parse_location_response;
use crate::response::parse_workspace_symbols_response;
use crate::uri::parse_uri;
