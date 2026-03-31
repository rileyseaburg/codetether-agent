//! LSP type definitions based on LSP 3.17 specification
//!
//! These types map directly to the LSP protocol types.

use anyhow::Result;
use lsp_types::{
    ClientCapabilities, CompletionItem, DiagnosticSeverity, DocumentSymbol, Location, Position,
    Range, ServerCapabilities, SymbolInformation, TextDocumentIdentifier, TextDocumentItem,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn};

/// LSP client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    /// Command to spawn the language server
    pub command: String,
    /// Arguments to pass to the language server
    #[serde(default)]
    pub args: Vec<String>,
    /// Root URI for the workspace
    pub root_uri: Option<String>,
    /// File extensions this server handles
    #[serde(default)]
    pub file_extensions: Vec<String>,
    /// Initialization options to pass to the server
    #[serde(default)]
    pub initialization_options: Option<Value>,
    /// Timeout for requests in milliseconds
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

fn default_timeout() -> u64 {
    30000
}

fn rust_timeout() -> u64 {
    120000
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            command: String::new(),
            args: Vec::new(),
            root_uri: None,
            file_extensions: Vec::new(),
            initialization_options: None,
            timeout_ms: default_timeout(),
        }
    }
}

/// JSON-RPC request for LSP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: i64,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl JsonRpcRequest {
    pub fn new(id: i64, method: &str, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        }
    }
}

/// JSON-RPC response for LSP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC notification for LSP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl JsonRpcNotification {
    pub fn new(method: &str, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
        }
    }
}

/// Initialize parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub process_id: Option<i64>,
    pub client_info: ClientInfo,
    pub locale: Option<String>,
    pub root_path: Option<String>,
    pub root_uri: Option<String>,
    pub initialization_options: Option<Value>,
    pub capabilities: ClientCapabilities,
    pub trace: Option<String>,
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFolder {
    pub uri: String,
    pub name: String,
}

/// Initialize result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<ServerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// DidOpenTextDocument parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidOpenTextDocumentParams {
    pub text_document: TextDocumentItem,
}

/// DidCloseTextDocument parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DidCloseTextDocumentParams {
    pub text_document: TextDocumentIdentifier,
}

/// DidChangeTextDocument parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DidChangeTextDocumentParams {
    pub text_document: VersionedTextDocumentIdentifier,
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TextDocumentContentChangeEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_length: Option<u32>,
    pub text: String,
}

/// Reference context for find references
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceContext {
    pub include_declaration: bool,
}

/// Reference parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub context: ReferenceContext,
}

/// Workspace symbol parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolParams {
    pub query: String,
}

/// LSP action result - unified response type for the tool
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LspActionResult {
    /// Go to definition result
    Definition { locations: Vec<LocationInfo> },
    /// Find references result
    References { locations: Vec<LocationInfo> },
    /// Hover result
    Hover {
        contents: String,
        range: Option<RangeInfo>,
    },
    /// Document symbols result
    DocumentSymbols { symbols: Vec<SymbolInfo> },
    /// Workspace symbols result
    WorkspaceSymbols { symbols: Vec<SymbolInfo> },
    /// Go to implementation result
    Implementation { locations: Vec<LocationInfo> },
    /// Completion result
    Completion { items: Vec<CompletionItemInfo> },
    /// Diagnostics result
    Diagnostics { diagnostics: Vec<DiagnosticInfo> },
    /// Error result
    Error { message: String },
}

/// Simplified location info for tool output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInfo {
    pub uri: String,
    pub range: RangeInfo,
}

impl From<Location> for LocationInfo {
    fn from(loc: Location) -> Self {
        Self {
            uri: loc.uri.to_string(),
            range: RangeInfo::from(loc.range),
        }
    }
}

/// Simplified range info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeInfo {
    pub start: PositionInfo,
    pub end: PositionInfo,
}

impl From<Range> for RangeInfo {
    fn from(range: Range) -> Self {
        Self {
            start: PositionInfo::from(range.start),
            end: PositionInfo::from(range.end),
        }
    }
}

/// Simplified position info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionInfo {
    pub line: u32,
    pub character: u32,
}

impl From<Position> for PositionInfo {
    fn from(pos: Position) -> Self {
        Self {
            line: pos.line,
            character: pos.character,
        }
    }
}

/// Simplified symbol info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<RangeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

impl From<DocumentSymbol> for SymbolInfo {
    fn from(sym: DocumentSymbol) -> Self {
        Self {
            name: sym.name,
            kind: format!("{:?}", sym.kind),
            detail: sym.detail,
            uri: None,
            range: Some(RangeInfo::from(sym.range)),
            container_name: None,
        }
    }
}

impl From<SymbolInformation> for SymbolInfo {
    fn from(sym: SymbolInformation) -> Self {
        Self {
            name: sym.name,
            kind: format!("{:?}", sym.kind),
            detail: None,
            uri: Some(sym.location.uri.to_string()),
            range: Some(RangeInfo::from(sym.location.range)),
            container_name: sym.container_name,
        }
    }
}

/// Simplified completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemInfo {
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_text: Option<String>,
}

impl From<CompletionItem> for CompletionItemInfo {
    fn from(item: CompletionItem) -> Self {
        Self {
            label: item.label,
            kind: item.kind.map(|k| format!("{:?}", k)),
            detail: item.detail,
            documentation: item.documentation.map(|d| match d {
                lsp_types::Documentation::String(s) => s,
                lsp_types::Documentation::MarkupContent(mc) => mc.value,
            }),
            insert_text: item.insert_text,
        }
    }
}

/// Simplified diagnostic info for tool output and proactive session context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticInfo {
    pub uri: String,
    pub range: RangeInfo,
    pub severity: Option<String>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

impl DiagnosticInfo {
    pub fn severity_rank(&self) -> u8 {
        match self.severity.as_deref() {
            Some("error") => 1,
            Some("warning") => 2,
            Some("information") => 3,
            Some("hint") => 4,
            _ => 5,
        }
    }
}

impl From<(String, lsp_types::Diagnostic)> for DiagnosticInfo {
    fn from((uri, diagnostic): (String, lsp_types::Diagnostic)) -> Self {
        let severity = diagnostic.severity.map(|severity| match severity {
            DiagnosticSeverity::ERROR => "error".to_string(),
            DiagnosticSeverity::WARNING => "warning".to_string(),
            DiagnosticSeverity::INFORMATION => "information".to_string(),
            DiagnosticSeverity::HINT => "hint".to_string(),
            _ => "unknown".to_string(),
        });

        let code = diagnostic.code.map(|code| match code {
            lsp_types::NumberOrString::Number(n) => n.to_string(),
            lsp_types::NumberOrString::String(s) => s,
        });

        Self {
            uri,
            range: RangeInfo::from(diagnostic.range),
            severity,
            code,
            source: diagnostic.source,
            message: diagnostic.message,
        }
    }
}

/// Known language server configurations
pub fn get_language_server_config(language: &str) -> Option<LspConfig> {
    match language {
        "rust" => Some(LspConfig {
            command: rust_analyzer_command(),
            args: rust_analyzer_args(),
            file_extensions: vec!["rs".to_string()],
            timeout_ms: rust_timeout(),
            ..Default::default()
        }),
        "typescript" | "javascript" => Some(LspConfig {
            command: "typescript-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            file_extensions: vec![
                "ts".to_string(),
                "tsx".to_string(),
                "js".to_string(),
                "jsx".to_string(),
            ],
            ..Default::default()
        }),
        "python" => Some(LspConfig {
            command: "pylsp".to_string(),
            args: vec![],
            file_extensions: vec!["py".to_string()],
            ..Default::default()
        }),
        "go" => Some(LspConfig {
            command: "gopls".to_string(),
            args: vec!["serve".to_string()],
            file_extensions: vec!["go".to_string()],
            ..Default::default()
        }),
        "c" | "cpp" | "c++" => Some(LspConfig {
            command: "clangd".to_string(),
            args: vec![],
            file_extensions: vec![
                "c".to_string(),
                "cpp".to_string(),
                "cc".to_string(),
                "cxx".to_string(),
                "h".to_string(),
                "hpp".to_string(),
            ],
            ..Default::default()
        }),
        _ => None,
    }
}

fn rust_analyzer_command() -> String {
    if which::which("rust-analyzer").is_ok() {
        "rust-analyzer".to_string()
    } else {
        "rustup".to_string()
    }
}

fn rust_analyzer_args() -> Vec<String> {
    if which::which("rust-analyzer").is_ok() {
        Vec::new()
    } else {
        vec![
            "run".to_string(),
            "stable".to_string(),
            "rust-analyzer".to_string(),
        ]
    }
}

/// Returns the install command for a language server binary, if known.
fn install_command_for(command: &str) -> Option<&'static [&'static str]> {
    match command {
        "rust-analyzer" => Some(&["rustup", "component", "add", "rust-analyzer"]),
        "typescript-language-server" => Some(&[
            "npm",
            "install",
            "-g",
            "typescript-language-server",
            "typescript",
        ]),
        "pylsp" => Some(&["pip", "install", "--user", "python-lsp-server"]),
        "gopls" => Some(&["go", "install", "golang.org/x/tools/gopls@latest"]),
        "clangd" => None, // system package manager varies
        _ => None,
    }
}

/// Ensure a language server binary is available, installing it if possible.
pub async fn ensure_server_installed(config: &LspConfig) -> Result<()> {
    // Check if the binary is already on PATH.
    if which::which(&config.command).is_ok() {
        return Ok(());
    }

    // rust-analyzer is commonly installed via rustup but may not be visible on PATH
    // in the current process environment. Fall back to `rustup run <toolchain> rust-analyzer`.
    if config.command == "rust-analyzer" {
        let rustup_status = tokio::process::Command::new("rustup")
            .args(["run", "stable", "rust-analyzer", "--version"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await;

        if let Ok(status) = rustup_status
            && status.success()
        {
            return Ok(());
        }
    }

    let Some(install_args) = install_command_for(&config.command) else {
        return Err(anyhow::anyhow!(
            "Language server '{}' not found and no auto-install available. Install it manually.",
            config.command,
        ));
    };

    info!(command = %config.command, "Language server not found, installing...");

    let output = tokio::process::Command::new(install_args[0])
        .args(&install_args[1..])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return Err(anyhow::anyhow!(
            "Failed to install '{}' (exit code {:?}). stdout: {} stderr: {}",
            config.command,
            output.status.code(),
            stdout,
            stderr,
        ));
    }

    // Verify installation succeeded
    if which::which(&config.command).is_err() {
        if config.command == "rust-analyzer" {
            let rustup_status = tokio::process::Command::new("rustup")
                .args(["run", "stable", "rust-analyzer", "--version"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .await;
            if let Ok(status) = rustup_status
                && status.success()
            {
                info!(command = %config.command, "Language server installed and available via rustup run stable");
                return Ok(());
            }
        }
        warn!(command = %config.command, "Install succeeded but binary still not found on PATH");
    } else {
        info!(command = %config.command, "Language server installed successfully");
    }

    Ok(())
}

/// Detect language from file extension
pub fn detect_language_from_path(path: &str) -> Option<&'static str> {
    let ext = path.rsplit('.').next()?;
    match ext {
        "rs" => Some("rust"),
        "ts" | "tsx" => Some("typescript"),
        "js" | "jsx" => Some("javascript"),
        "py" => Some("python"),
        "go" => Some("go"),
        "c" => Some("c"),
        "cpp" | "cc" | "cxx" => Some("cpp"),
        "h" => Some("c"),
        "hpp" => Some("cpp"),
        _ => None,
    }
}

/// Built-in linter server configurations.
/// Returns an `LspConfig` for well-known linter language servers.
pub fn get_linter_server_config(name: &str) -> Option<LspConfig> {
    match name {
        "eslint" => Some(LspConfig {
            command: "vscode-eslint-language-server".to_string(),
            args: vec!["--stdio".to_string()],
            file_extensions: vec![
                "js".to_string(),
                "jsx".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "mjs".to_string(),
                "cjs".to_string(),
            ],
            ..Default::default()
        }),
        "biome" => Some(LspConfig {
            command: "biome".to_string(),
            args: vec!["lsp-proxy".to_string()],
            file_extensions: vec![
                "js".to_string(),
                "jsx".to_string(),
                "ts".to_string(),
                "tsx".to_string(),
                "json".to_string(),
                "css".to_string(),
            ],
            ..Default::default()
        }),
        "ruff" => Some(LspConfig {
            command: "ruff".to_string(),
            args: vec!["server".to_string()],
            file_extensions: vec!["py".to_string(), "pyi".to_string()],
            ..Default::default()
        }),
        "stylelint" => Some(LspConfig {
            command: "stylelint-lsp".to_string(),
            args: vec!["--stdio".to_string()],
            file_extensions: vec!["css".to_string(), "scss".to_string(), "less".to_string()],
            ..Default::default()
        }),
        _ => None,
    }
}

/// Convert a config `LspServerEntry` into an `LspConfig`.
impl LspConfig {
    pub fn from_server_entry(
        entry: &crate::config::LspServerEntry,
        root_uri: Option<String>,
    ) -> Self {
        Self {
            command: entry.command.clone(),
            args: entry.args.clone(),
            root_uri,
            file_extensions: entry.file_extensions.clone(),
            initialization_options: entry.initialization_options.clone(),
            timeout_ms: entry.timeout_ms,
        }
    }

    pub fn from_linter_entry(
        name: &str,
        entry: &crate::config::LspLinterEntry,
        root_uri: Option<String>,
    ) -> Option<Self> {
        // Start from the built-in config if the linter name is known,
        // then overlay any user overrides.
        let mut base = if let Some(builtin) = get_linter_server_config(name) {
            builtin
        } else {
            // Fully custom linter — command is required.
            let command = entry.command.as_ref()?;
            LspConfig {
                command: command.clone(),
                ..Default::default()
            }
        };

        // User overrides
        if let Some(cmd) = &entry.command {
            base.command = cmd.clone();
        }
        if !entry.args.is_empty() {
            base.args = entry.args.clone();
        }
        if !entry.file_extensions.is_empty() {
            base.file_extensions = entry.file_extensions.clone();
        }
        if entry.initialization_options.is_some() {
            base.initialization_options = entry.initialization_options.clone();
        }
        base.root_uri = root_uri;
        Some(base)
    }
}

/// Returns all file extensions handled by a named linter (built-in defaults).
pub fn linter_extensions(name: &str) -> &'static [&'static str] {
    match name {
        "eslint" => &["js", "jsx", "ts", "tsx", "mjs", "cjs"],
        "biome" => &["js", "jsx", "ts", "tsx", "json", "css"],
        "ruff" => &["py", "pyi"],
        "stylelint" => &["css", "scss", "less"],
        _ => &[],
    }
}
