//! LSP Client - manages language server connections and operations
//!
//! Provides high-level API for LSP operations:
//! - Initialize/shutdown lifecycle
//! - Document synchronization
//! - Code intelligence (definition, references, hover, etc.)

use super::transport::LspTransport;
use super::types::*;
use anyhow::Result;
use lsp_types::{
    ClientCapabilities, CompletionContext, CompletionParams, CompletionTriggerKind,
    DocumentSymbolParams, HoverParams, Position, TextDocumentIdentifier, TextDocumentItem,
    TextDocumentPositionParams,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// LSP Client for a single language server
pub struct LspClient {
    transport: LspTransport,
    config: LspConfig,
    server_capabilities: RwLock<Option<lsp_types::ServerCapabilities>>,
    /// Track open documents with their versions
    open_documents: RwLock<HashMap<String, i32>>,
}

impl LspClient {
    /// Create a new LSP client with the given configuration
    pub async fn new(config: LspConfig) -> Result<Self> {
        super::types::ensure_server_installed(&config).await?;

        let transport =
            LspTransport::spawn(&config.command, &config.args, config.timeout_ms).await?;

        Ok(Self {
            transport,
            config,
            server_capabilities: RwLock::new(None),
            open_documents: RwLock::new(HashMap::new()),
        })
    }

    /// Create an LSP client for a specific language
    pub async fn for_language(language: &str, root_uri: Option<String>) -> Result<Self> {
        let mut config = get_language_server_config(language)
            .ok_or_else(|| anyhow::anyhow!("Unknown language: {}", language))?;
        config.root_uri = root_uri;
        Self::new(config).await
    }

    /// Initialize the language server
    pub async fn initialize(&self) -> Result<()> {
        let root_uri = self.config.root_uri.clone();

        let params = InitializeParams {
            process_id: Some(std::process::id() as i64),
            client_info: ClientInfo {
                name: "codetether".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            locale: None,
            root_path: None,
            root_uri: root_uri.clone(),
            initialization_options: self.config.initialization_options.clone(),
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: None,
        };

        let response = self
            .transport
            .request("initialize", Some(serde_json::to_value(params)?))
            .await?;

        if let Some(error) = response.error {
            return Err(anyhow::anyhow!("LSP initialize error: {}", error.message));
        }

        if let Some(result) = response.result {
            let init_result: InitializeResult = serde_json::from_value(result)?;
            *self.server_capabilities.write().await = Some(init_result.capabilities);
            info!(server_info = ?init_result.server_info, "LSP server initialized");
        }

        self.transport.notify("initialized", None).await?;
        self.transport.set_initialized(true);

        Ok(())
    }

    /// Shutdown the language server
    #[allow(dead_code)]
    pub async fn shutdown(&self) -> Result<()> {
        let response = self.transport.request("shutdown", None).await?;

        if let Some(error) = response.error {
            warn!("LSP shutdown error: {}", error.message);
        }

        self.transport.notify("exit", None).await?;
        info!("LSP server shutdown complete");

        Ok(())
    }

    /// Open a text document
    pub async fn open_document(&self, path: &Path, content: &str) -> Result<()> {
        let uri = path_to_uri(path);
        let language_id = detect_language_from_path(path.to_string_lossy().as_ref())
            .unwrap_or("plaintext")
            .to_string();

        let text_document = TextDocumentItem {
            uri: parse_uri(&uri)?,
            language_id,
            version: 1,
            text: content.to_string(),
        };

        let params = DidOpenTextDocumentParams { text_document };
        self.transport
            .notify("textDocument/didOpen", Some(serde_json::to_value(params)?))
            .await?;

        self.open_documents.write().await.insert(uri, 1);
        debug!(path = %path.display(), "Opened document");

        Ok(())
    }

    /// Close a text document
    #[allow(dead_code)]
    pub async fn close_document(&self, path: &Path) -> Result<()> {
        let uri = path_to_uri(path);

        let text_document = TextDocumentIdentifier {
            uri: parse_uri(&uri)?,
        };

        let params = DidCloseTextDocumentParams { text_document };
        self.transport
            .notify("textDocument/didClose", Some(serde_json::to_value(params)?))
            .await?;

        self.open_documents.write().await.remove(&uri);
        debug!(path = %path.display(), "Closed document");

        Ok(())
    }

    /// Update a text document
    #[allow(dead_code)]
    pub async fn change_document(&self, path: &Path, content: &str) -> Result<()> {
        let uri = path_to_uri(path);
        let mut open_docs = self.open_documents.write().await;

        let version = open_docs.entry(uri.clone()).or_insert(0);
        *version += 1;

        let text_document = VersionedTextDocumentIdentifier {
            uri,
            version: *version,
        };

        let content_changes = vec![super::types::TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: content.to_string(),
        }];

        let params = DidChangeTextDocumentParams {
            text_document,
            content_changes,
        };

        self.transport
            .notify(
                "textDocument/didChange",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        debug!(path = %path.display(), version = *version, "Changed document");

        Ok(())
    }

    /// Go to definition
    pub async fn go_to_definition(
        &self,
        path: &Path,
        line: u32,
        character: u32,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line.saturating_sub(1), "character": character.saturating_sub(1) },
        });

        let response = self
            .transport
            .request("textDocument/definition", Some(params))
            .await?;

        parse_location_response(response, "definition")
    }

    /// Find references
    pub async fn find_references(
        &self,
        path: &Path,
        line: u32,
        character: u32,
        include_declaration: bool,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = ReferenceParams {
            text_document: TextDocumentIdentifier {
                uri: parse_uri(&uri)?,
            },
            position: Position {
                line: line.saturating_sub(1),
                character: character.saturating_sub(1),
            },
            context: ReferenceContext {
                include_declaration,
            },
        };

        let response = self
            .transport
            .request(
                "textDocument/references",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        parse_location_response(response, "references")
    }

    /// Get hover information
    pub async fn hover(&self, path: &Path, line: u32, character: u32) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: parse_uri(&uri)?,
                },
                position: Position {
                    line: line.saturating_sub(1),
                    character: character.saturating_sub(1),
                },
            },
            work_done_progress_params: Default::default(),
        };

        let response = self
            .transport
            .request("textDocument/hover", Some(serde_json::to_value(params)?))
            .await?;

        parse_hover_response(response)
    }

    /// Get document symbols
    pub async fn document_symbols(&self, path: &Path) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = DocumentSymbolParams {
            text_document: TextDocumentIdentifier {
                uri: parse_uri(&uri)?,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };

        let response = self
            .transport
            .request(
                "textDocument/documentSymbol",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        parse_document_symbols_response(response)
    }

    /// Search workspace symbols
    pub async fn workspace_symbols(&self, query: &str) -> Result<LspActionResult> {
        let params = WorkspaceSymbolParams {
            query: query.to_string(),
        };

        let response = self
            .transport
            .request("workspace/symbol", Some(serde_json::to_value(params)?))
            .await?;

        parse_workspace_symbols_response(response)
    }

    /// Go to implementation
    pub async fn go_to_implementation(
        &self,
        path: &Path,
        line: u32,
        character: u32,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line.saturating_sub(1), "character": character.saturating_sub(1) },
        });

        let response = self
            .transport
            .request("textDocument/implementation", Some(params))
            .await?;

        parse_location_response(response, "implementation")
    }

    /// Get code completions
    pub async fn completion(
        &self,
        path: &Path,
        line: u32,
        character: u32,
    ) -> Result<LspActionResult> {
        let uri = path_to_uri(path);
        self.ensure_document_open(path).await?;

        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: parse_uri(&uri)?,
                },
                position: Position {
                    line: line.saturating_sub(1),
                    character: character.saturating_sub(1),
                },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: Some(CompletionContext {
                trigger_kind: CompletionTriggerKind::INVOKED,
                trigger_character: None,
            }),
        };

        let response = self
            .transport
            .request(
                "textDocument/completion",
                Some(serde_json::to_value(params)?),
            )
            .await?;

        parse_completion_response(response)
    }

    /// Return the most recent LSP diagnostics for a file after ensuring the document is open.
    pub async fn diagnostics(&self, path: &Path) -> Result<LspActionResult> {
        self.ensure_document_open(path).await?;
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;

        let uri = path_to_uri(path);
        let snapshot = self.transport.diagnostics_snapshot().await;
        let diagnostics = snapshot
            .get(&uri)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|diagnostic| DiagnosticInfo::from((uri.clone(), diagnostic)))
            .collect();

        Ok(LspActionResult::Diagnostics { diagnostics })
    }

    /// Ensure a document is open (open it if not already)
    async fn ensure_document_open(&self, path: &Path) -> Result<()> {
        let uri = path_to_uri(path);
        if !self.open_documents.read().await.contains_key(&uri) {
            let content = tokio::fs::read_to_string(path).await?;
            self.open_document(path, &content).await?;
        }
        Ok(())
    }

    /// Get the server capabilities
    #[allow(dead_code)]
    pub async fn capabilities(&self) -> Option<lsp_types::ServerCapabilities> {
        self.server_capabilities.read().await.clone()
    }

    /// Check if this client handles the given file extension
    #[allow(dead_code)]
    pub fn handles_file(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        self.config.file_extensions.iter().any(|fe| fe == ext)
    }

    /// Check if this client handles a language by name
    #[allow(dead_code)]
    pub fn handles_language(&self, language: &str) -> bool {
        let extensions = match language {
            "rust" => &["rs"][..],
            "typescript" => &["ts", "tsx"],
            "javascript" => &["js", "jsx"],
            "python" => &["py"],
            "go" => &["go"],
            "c" => &["c", "h"],
            "cpp" => &["cpp", "cc", "cxx", "hpp", "h"],
            _ => &[],
        };

        extensions
            .iter()
            .any(|ext| self.config.file_extensions.iter().any(|fe| fe == *ext))
    }
}

/// Convert a file path to a file:// URI
fn path_to_uri(path: &Path) -> String {
    let absolute = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    format!("file://{}", absolute.display())
}

/// Parse a string URI into an lsp_types::Uri
fn parse_uri(uri_str: &str) -> Result<lsp_types::Uri> {
    uri_str
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid URI: {e}"))
}

/// Parse a location response (definition, references, implementation)
fn parse_location_response(response: JsonRpcResponse, _operation: &str) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::Definition { locations: vec![] });
    };

    if let Ok(loc) = serde_json::from_value::<lsp_types::Location>(result.clone()) {
        return Ok(LspActionResult::Definition {
            locations: vec![LocationInfo::from(loc)],
        });
    }

    if let Ok(locs) = serde_json::from_value::<Vec<lsp_types::Location>>(result.clone()) {
        return Ok(LspActionResult::Definition {
            locations: locs.into_iter().map(LocationInfo::from).collect(),
        });
    }

    if let Ok(links) = serde_json::from_value::<Vec<lsp_types::LocationLink>>(result) {
        return Ok(LspActionResult::Definition {
            locations: links
                .into_iter()
                .map(|link| LocationInfo {
                    uri: link.target_uri.to_string(),
                    range: RangeInfo::from(link.target_selection_range),
                })
                .collect(),
        });
    }

    Ok(LspActionResult::Definition { locations: vec![] })
}

/// Parse a hover response
fn parse_hover_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::Hover {
            contents: String::new(),
            range: None,
        });
    };

    if result.is_null() {
        return Ok(LspActionResult::Hover {
            contents: "No hover information available".to_string(),
            range: None,
        });
    }

    let hover: lsp_types::Hover = serde_json::from_value(result)?;

    let contents = match hover.contents {
        lsp_types::HoverContents::Scalar(markup) => match markup {
            lsp_types::MarkedString::String(s) => s,
            lsp_types::MarkedString::LanguageString(ls) => ls.value,
        },
        lsp_types::HoverContents::Array(markups) => markups
            .into_iter()
            .map(|m| match m {
                lsp_types::MarkedString::String(s) => s,
                lsp_types::MarkedString::LanguageString(ls) => ls.value,
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
        lsp_types::HoverContents::Markup(markup) => markup.value,
    };

    Ok(LspActionResult::Hover {
        contents,
        range: hover.range.map(RangeInfo::from),
    })
}

/// Parse a document symbols response
fn parse_document_symbols_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::DocumentSymbols { symbols: vec![] });
    };

    if result.is_null() {
        return Ok(LspActionResult::DocumentSymbols { symbols: vec![] });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::DocumentSymbol>>(result.clone()) {
        return Ok(LspActionResult::DocumentSymbols {
            symbols: symbols.into_iter().map(SymbolInfo::from).collect(),
        });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::SymbolInformation>>(result) {
        return Ok(LspActionResult::DocumentSymbols {
            symbols: symbols.into_iter().map(SymbolInfo::from).collect(),
        });
    }

    Ok(LspActionResult::DocumentSymbols { symbols: vec![] })
}

/// Parse a workspace symbols response
fn parse_workspace_symbols_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::WorkspaceSymbols { symbols: vec![] });
    };

    if result.is_null() {
        return Ok(LspActionResult::WorkspaceSymbols { symbols: vec![] });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::SymbolInformation>>(result.clone())
    {
        return Ok(LspActionResult::WorkspaceSymbols {
            symbols: symbols.into_iter().map(SymbolInfo::from).collect(),
        });
    }

    if let Ok(symbols) = serde_json::from_value::<Vec<lsp_types::WorkspaceSymbol>>(result) {
        return Ok(LspActionResult::WorkspaceSymbols {
            symbols: symbols
                .into_iter()
                .map(|s| {
                    let (uri, range) = match s.location {
                        lsp_types::OneOf::Left(loc) => {
                            (loc.uri.to_string(), Some(RangeInfo::from(loc.range)))
                        }
                        lsp_types::OneOf::Right(wl) => (wl.uri.to_string(), None),
                    };
                    SymbolInfo {
                        name: s.name,
                        kind: format!("{:?}", s.kind),
                        detail: None,
                        uri: Some(uri),
                        range,
                        container_name: s.container_name,
                    }
                })
                .collect(),
        });
    }

    Ok(LspActionResult::WorkspaceSymbols { symbols: vec![] })
}

/// Parse a completion response
fn parse_completion_response(response: JsonRpcResponse) -> Result<LspActionResult> {
    if let Some(error) = response.error {
        return Ok(LspActionResult::Error {
            message: error.message,
        });
    }

    let Some(result) = response.result else {
        return Ok(LspActionResult::Completion { items: vec![] });
    };

    if result.is_null() {
        return Ok(LspActionResult::Completion { items: vec![] });
    }

    if let Ok(list) = serde_json::from_value::<lsp_types::CompletionList>(result.clone()) {
        return Ok(LspActionResult::Completion {
            items: list
                .items
                .into_iter()
                .map(CompletionItemInfo::from)
                .collect(),
        });
    }

    if let Ok(items) = serde_json::from_value::<Vec<lsp_types::CompletionItem>>(result) {
        return Ok(LspActionResult::Completion {
            items: items.into_iter().map(CompletionItemInfo::from).collect(),
        });
    }

    Ok(LspActionResult::Completion { items: vec![] })
}

/// LSP Manager - manages multiple language server connections
pub struct LspManager {
    clients: RwLock<HashMap<String, Arc<LspClient>>>,
    /// Linter clients keyed by linter name (e.g. "eslint", "ruff").
    /// These are only queried for diagnostics, not completions/definitions.
    linter_clients: RwLock<HashMap<String, Arc<LspClient>>>,
    root_uri: Option<String>,
    /// User-supplied LSP settings from config.
    lsp_settings: Option<crate::config::LspSettings>,
}

impl LspManager {
    /// Create a new LSP manager
    pub fn new(root_uri: Option<String>) -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            linter_clients: RwLock::new(HashMap::new()),
            root_uri,
            lsp_settings: None,
        }
    }

    /// Create a new LSP manager with config-driven settings.
    pub fn with_config(root_uri: Option<String>, settings: crate::config::LspSettings) -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            linter_clients: RwLock::new(HashMap::new()),
            root_uri,
            lsp_settings: Some(settings),
        }
    }

    /// Get or create a client for the given language
    pub async fn get_client(&self, language: &str) -> Result<Arc<LspClient>> {
        {
            let clients = self.clients.read().await;
            if let Some(client) = clients.get(language) {
                return Ok(Arc::clone(client));
            }
        }

        let client = if let Some(settings) = &self.lsp_settings {
            if let Some(entry) = settings.servers.get(language) {
                let config = LspConfig::from_server_entry(entry, self.root_uri.clone());
                LspClient::new(config).await?
            } else {
                LspClient::for_language(language, self.root_uri.clone()).await?
            }
        } else {
            LspClient::for_language(language, self.root_uri.clone()).await?
        };
        client.initialize().await?;

        let client = Arc::new(client);
        self.clients
            .write()
            .await
            .insert(language.to_string(), Arc::clone(&client));

        Ok(client)
    }

    /// Get a client for a file path (detects language from extension)
    pub async fn get_client_for_file(&self, path: &Path) -> Result<Arc<LspClient>> {
        let language = detect_language_from_path(path.to_string_lossy().as_ref())
            .ok_or_else(|| anyhow::anyhow!("Unknown language for file: {}", path.display()))?;
        self.get_client(language).await
    }

    /// Check if any registered client handles the given file.
    #[allow(dead_code)]
    pub async fn handles_file(&self, path: &Path) -> bool {
        let clients = self.clients.read().await;
        clients.values().any(|c| c.handles_file(path))
    }

    /// Get capabilities for a specific language server.
    #[allow(dead_code)]
    pub async fn capabilities_for(&self, language: &str) -> Option<lsp_types::ServerCapabilities> {
        let clients = self.clients.read().await;
        if let Some(client) = clients.get(language) {
            client.capabilities().await
        } else {
            None
        }
    }

    /// Close a document across all relevant clients.
    #[allow(dead_code)]
    pub async fn close_document(&self, path: &Path) -> Result<()> {
        if let Ok(client) = self.get_client_for_file(path).await {
            client.close_document(path).await?;
        }
        Ok(())
    }

    /// Notify clients of a document change.
    #[allow(dead_code)]
    pub async fn change_document(&self, path: &Path, content: &str) -> Result<()> {
        if let Ok(client) = self.get_client_for_file(path).await {
            client.change_document(path, content).await?;
        }
        Ok(())
    }

    /// Shutdown all clients
    #[allow(dead_code)]
    pub async fn shutdown_all(&self) {
        let clients = self.clients.read().await;
        for (lang, client) in clients.iter() {
            if let Err(e) = client.shutdown().await {
                warn!("Failed to shutdown {} language server: {}", lang, e);
            }
        }
        let linters = self.linter_clients.read().await;
        for (name, client) in linters.iter() {
            if let Err(e) = client.shutdown().await {
                warn!("Failed to shutdown {} linter server: {}", name, e);
            }
        }
    }

    /// Get or start a linter client by name (e.g. "eslint", "ruff").
    /// Returns `None` if the linter is not configured or its binary is missing.
    pub async fn get_linter_client(&self, name: &str) -> Result<Option<Arc<LspClient>>> {
        // Already running?
        {
            let linters = self.linter_clients.read().await;
            if let Some(client) = linters.get(name) {
                return Ok(Some(Arc::clone(client)));
            }
        }

        // Resolve config
        let lsp_config = if let Some(settings) = &self.lsp_settings {
            if let Some(entry) = settings.linters.get(name) {
                if !entry.enabled {
                    return Ok(None);
                }
                LspConfig::from_linter_entry(name, entry, self.root_uri.clone())
            } else if !settings.disable_builtin_linters {
                // Not explicitly configured — try built-in
                if let Some(mut cfg) = get_linter_server_config(name) {
                    cfg.root_uri = self.root_uri.clone();
                    Some(cfg)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            // No config provided — use built-in defaults
            if let Some(mut cfg) = get_linter_server_config(name) {
                cfg.root_uri = self.root_uri.clone();
                Some(cfg)
            } else {
                None
            }
        };

        let Some(config) = lsp_config else {
            return Ok(None);
        };

        // Try to start; if the binary is missing, return None instead of hard error
        let client = match LspClient::new(config).await {
            Ok(c) => c,
            Err(e) => {
                debug!(linter = name, error = %e, "Linter server not available");
                return Ok(None);
            }
        };
        if let Err(e) = client.initialize().await {
            warn!(linter = name, error = %e, "Linter server failed to initialize");
            return Ok(None);
        }

        let client = Arc::new(client);
        self.linter_clients
            .write()
            .await
            .insert(name.to_string(), Arc::clone(&client));
        info!(linter = name, "Linter server started");
        Ok(Some(client))
    }

    /// Collect diagnostics from all applicable linter servers for a file.
    /// Returns an empty vec if no linters match the file extension.
    pub async fn linter_diagnostics(&self, path: &Path) -> Vec<DiagnosticInfo> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Determine which linters apply based on file extension
        let linter_names: Vec<String> = if let Some(settings) = &self.lsp_settings {
            settings
                .linters
                .iter()
                .filter(|(_, entry)| {
                    entry.enabled
                        && (entry.file_extensions.iter().any(|e| e == ext)
                            || entry.file_extensions.is_empty())
                })
                .map(|(name, _)| name.clone())
                .collect()
        } else {
            // Auto-detect: try known linters whose extensions match
            let mut names = Vec::new();
            for candidate in &["eslint", "biome", "ruff", "stylelint"] {
                if linter_extensions(candidate).contains(&ext) {
                    names.push((*candidate).to_string());
                }
            }
            names
        };

        let mut all_diagnostics = Vec::new();
        for name in &linter_names {
            match self.get_linter_client(name).await {
                Ok(Some(client)) => match client.diagnostics(path).await {
                    Ok(LspActionResult::Diagnostics { diagnostics }) => {
                        all_diagnostics.extend(diagnostics);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        debug!(linter = %name, error = %e, "Linter diagnostics failed");
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    debug!(linter = %name, error = %e, "Failed to get linter client");
                }
            }
        }
        all_diagnostics
    }
}

impl Default for LspManager {
    fn default() -> Self {
        Self::new(None)
    }
}
