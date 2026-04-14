//! LSP tool: Language Server Protocol operations

use crate::lsp::{LspActionResult, LspManager, detect_language_from_path};

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;

/// Global LSP managers keyed by workspace root.
static LSP_MANAGERS: std::sync::OnceLock<Arc<RwLock<HashMap<String, (u64, Arc<LspManager>)>>>> =
    std::sync::OnceLock::new();
static LSP_MANAGER_ACCESS: AtomicU64 = AtomicU64::new(0);
const MAX_LSP_MANAGERS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LspOperation {
    GoToDefinition,
    FindReferences,
    Hover,
    DocumentSymbol,
    WorkspaceSymbol,
    GoToImplementation,
    Completion,
    Diagnostics,
}

impl LspOperation {
    fn parse(action: &str) -> Option<Self> {
        match action {
            "goToDefinition" | "go-to-definition" | "go_to_definition" => {
                Some(Self::GoToDefinition)
            }
            "findReferences" | "find-references" | "find_references" => Some(Self::FindReferences),
            "hover" => Some(Self::Hover),
            "documentSymbol" | "document-symbol" | "document_symbol" => Some(Self::DocumentSymbol),
            "workspaceSymbol" | "workspace-symbol" | "workspace_symbol" => {
                Some(Self::WorkspaceSymbol)
            }
            "goToImplementation" | "go-to-implementation" | "go_to_implementation" => {
                Some(Self::GoToImplementation)
            }
            "completion" => Some(Self::Completion),
            "diagnostics" => Some(Self::Diagnostics),
            _ => None,
        }
    }

    fn requires_position(self) -> bool {
        match self {
            Self::GoToDefinition
            | Self::FindReferences
            | Self::Hover
            | Self::GoToImplementation
            | Self::Completion => true,
            Self::DocumentSymbol | Self::WorkspaceSymbol | Self::Diagnostics => false,
        }
    }

    fn canonical_name(self) -> &'static str {
        match self {
            Self::GoToDefinition => "goToDefinition",
            Self::FindReferences => "findReferences",
            Self::Hover => "hover",
            Self::DocumentSymbol => "documentSymbol",
            Self::WorkspaceSymbol => "workspaceSymbol",
            Self::GoToImplementation => "goToImplementation",
            Self::Completion => "completion",
            Self::Diagnostics => "diagnostics",
        }
    }
}

fn get_file_path_arg(args: &Value) -> Option<&str> {
    args["file_path"].as_str().or_else(|| args["path"].as_str())
}

fn action_from_command(command: &str) -> Option<&'static str> {
    match command {
        "textDocument/definition" => Some("goToDefinition"),
        "textDocument/references" => Some("findReferences"),
        "textDocument/hover" => Some("hover"),
        "textDocument/documentSymbol" => Some("documentSymbol"),
        "workspace/symbol" => Some("workspaceSymbol"),
        "textDocument/implementation" => Some("goToImplementation"),
        "textDocument/completion" => Some("completion"),
        _ => None,
    }
}

fn resolve_action_raw(args: &Value) -> Result<String> {
    if let Some(action) = args["action"].as_str() {
        return Ok(action.to_string());
    }

    if let Some(command) = args["command"].as_str() {
        if let Some(mapped) = action_from_command(command) {
            return Ok(mapped.to_string());
        }

        return Err(anyhow::anyhow!(
            "Unsupported lsp command: {command}. Use action with one of: \
             goToDefinition, findReferences, hover, documentSymbol, workspaceSymbol, \
             goToImplementation, completion, diagnostics"
        ));
    }

    Err(anyhow::anyhow!("action is required"))
}

/// LSP Tool for performing Language Server Protocol operations
pub struct LspTool {
    root_uri: Option<String>,
    lsp_settings: Option<crate::config::LspSettings>,
}

impl LspTool {
    pub fn new() -> Self {
        Self {
            root_uri: None,
            lsp_settings: None,
        }
    }

    /// Create with a specific workspace root
    pub fn with_root(root_uri: String) -> Self {
        Self {
            root_uri: Some(root_uri),
            lsp_settings: None,
        }
    }

    /// Create with workspace root and config-driven LSP settings.
    pub fn with_config(root_uri: Option<String>, settings: crate::config::LspSettings) -> Self {
        Self {
            root_uri,
            lsp_settings: Some(settings),
        }
    }

    fn manager_key(&self) -> String {
        self.root_uri
            .clone()
            .unwrap_or_else(|| "__default__".to_string())
    }

    /// Shutdown all LSP clients, releasing resources.
    #[allow(dead_code)]
    pub async fn shutdown_all(&self) {
        let cell = LSP_MANAGERS.get_or_init(|| Arc::new(RwLock::new(HashMap::new())));
        let managers = {
            let mut guard = cell.write().await;
            let managers = guard
                .values()
                .map(|(_, manager)| Arc::clone(manager))
                .collect::<Vec<_>>();
            guard.clear();
            managers
        };
        for manager in managers {
            manager.shutdown_all().await;
        }
    }

    /// Get or initialize the LSP manager
    pub async fn get_manager(&self) -> Arc<LspManager> {
        let access = LSP_MANAGER_ACCESS.fetch_add(1, Ordering::Relaxed);
        let key = self.manager_key();
        let cell = LSP_MANAGERS.get_or_init(|| Arc::new(RwLock::new(HashMap::new())));

        {
            let mut guard = cell.write().await;
            if let Some((last_access, manager)) = guard.get_mut(&key) {
                *last_access = access;
                return Arc::clone(manager);
            }
        }

        let manager = if let Some(settings) = &self.lsp_settings {
            Arc::new(LspManager::with_config(
                self.root_uri.clone(),
                settings.clone(),
            ))
        } else {
            match crate::config::Config::load().await {
                Ok(config) if has_lsp_settings(&config.lsp) => {
                    Arc::new(LspManager::with_config(self.root_uri.clone(), config.lsp))
                }
                _ => Arc::new(LspManager::new(self.root_uri.clone())),
            }
        };

        let evicted_manager = {
            let mut guard = cell.write().await;
            if let Some((last_access, existing_manager)) = guard.get_mut(&key) {
                *last_access = access;
                return Arc::clone(existing_manager);
            }

            let evicted_manager = if guard.len() >= MAX_LSP_MANAGERS {
                let evicted_key = guard
                    .iter()
                    .min_by_key(|(_, (last_access, _))| *last_access)
                    .map(|(evicted_key, _)| evicted_key.clone());
                evicted_key
                    .and_then(|evicted_key| guard.remove(&evicted_key))
                    .map(|(_, evicted_manager)| evicted_manager)
            } else {
                None
            };

            guard.insert(key, (access, Arc::clone(&manager)));
            evicted_manager
        };

        if let Some(evicted_manager) = evicted_manager
            && Arc::strong_count(&evicted_manager) == 1
        {
            evicted_manager.shutdown_all().await;
        }

        manager
    }
}

impl Default for LspTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for LspTool {
    fn id(&self) -> &str {
        "lsp"
    }

    fn name(&self) -> &str {
        "LSP Tool"
    }

    fn description(&self) -> &str {
        "Perform Language Server Protocol (LSP) operations such as go-to-definition, find-references, hover, document-symbol, workspace-symbol, diagnostics, and more. This tool enables AI agents to query language servers for code intelligence features. Supports rust-analyzer, typescript-language-server, pylsp, gopls, and clangd."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "The LSP operation to perform",
                    "enum": [
                        "goToDefinition",
                        "go-to-definition",
                        "go_to_definition",
                        "findReferences",
                        "find-references",
                        "find_references",
                        "hover",
                        "documentSymbol",
                        "document-symbol",
                        "document_symbol",
                        "workspaceSymbol",
                        "workspace-symbol",
                        "workspace_symbol",
                        "goToImplementation",
                        "go-to-implementation",
                        "go_to_implementation",
                        "completion",
                        "diagnostics"
                    ]
                },
                "file_path": {
                    "type": "string",
                    "description": "The absolute or relative path to the file"
                },
                "path": {
                    "type": "string",
                    "description": "Alias for file_path"
                },
                "line": {
                    "type": "integer",
                    "description": "The line number (1-based, as shown in editors)",
                    "minimum": 1
                },
                "column": {
                    "type": "integer",
                    "description": "The character offset/column (1-based, as shown in editors)",
                    "minimum": 1
                },
                "query": {
                    "type": "string",
                    "description": "Search query for workspaceSymbol action"
                },
                "include_declaration": {
                    "type": "boolean",
                    "description": "For findReferences: include the declaration in results",
                    "default": true
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let action_raw = resolve_action_raw(&args)?;
        let action = LspOperation::parse(&action_raw)
            .ok_or_else(|| anyhow::anyhow!("Unknown action: {}", action_raw))?;

        let manager = self.get_manager().await;

        if action == LspOperation::WorkspaceSymbol {
            let query = args["query"].as_str().unwrap_or("");
            let language = get_file_path_arg(&args).and_then(detect_language_from_path);

            let client = if let Some(lang) = language {
                manager.get_client(lang).await?
            } else {
                manager.get_client("rust").await?
            };

            let result = client.workspace_symbols(query).await?;
            return format_result(result);
        }

        let file_path = get_file_path_arg(&args).ok_or_else(|| {
            anyhow::anyhow!(
                "file_path is required (or use path) for action: {}",
                action_raw
            )
        })?;
        let path = Path::new(file_path);

        let client = manager.get_client_for_file(path).await?;

        let line = args["line"].as_u64().map(|l| l as u32);
        let column = args["column"].as_u64().map(|c| c as u32);

        if action.requires_position() && (line.is_none() || column.is_none()) {
            return Ok(ToolResult::error(format!(
                "line and column are required for action: {}",
                action.canonical_name()
            )));
        }

        let result = match action {
            LspOperation::GoToDefinition => {
                client
                    .go_to_definition(
                        path,
                        line.expect("line required"),
                        column.expect("column required"),
                    )
                    .await?
            }
            LspOperation::FindReferences => {
                let include_decl = args["include_declaration"].as_bool().unwrap_or(true);
                client
                    .find_references(
                        path,
                        line.expect("line required"),
                        column.expect("column required"),
                        include_decl,
                    )
                    .await?
            }
            LspOperation::Hover => {
                client
                    .hover(
                        path,
                        line.expect("line required"),
                        column.expect("column required"),
                    )
                    .await?
            }
            LspOperation::DocumentSymbol => client.document_symbols(path).await?,
            LspOperation::GoToImplementation => {
                client
                    .go_to_implementation(
                        path,
                        line.expect("line required"),
                        column.expect("column required"),
                    )
                    .await?
            }
            LspOperation::Completion => {
                client
                    .completion(
                        path,
                        line.expect("line required"),
                        column.expect("column required"),
                    )
                    .await?
            }
            LspOperation::Diagnostics => {
                let mut result = client.diagnostics(path).await?;
                // Merge diagnostics from any applicable linter servers
                let linter_diags = manager.linter_diagnostics(path).await;
                if !linter_diags.is_empty() {
                    if let LspActionResult::Diagnostics {
                        ref mut diagnostics,
                    } = result
                    {
                        diagnostics.extend(linter_diags);
                    }
                }
                result
            }
            LspOperation::WorkspaceSymbol => {
                return Ok(ToolResult::error(format!(
                    "Action {} is handled separately",
                    action.canonical_name()
                )));
            }
        };

        format_result(result)
    }
}

/// Format LSP result as tool output
fn format_result(result: LspActionResult) -> Result<ToolResult> {
    let output = match result {
        LspActionResult::Definition { locations } => {
            if locations.is_empty() {
                "No definition found".to_string()
            } else {
                let mut out = format!("Found {} definition(s):\n\n", locations.len());
                for loc in locations {
                    let uri = loc.uri;
                    let range = loc.range;
                    out.push_str(&format!(
                        "  {}:{}:{}\n",
                        uri.trim_start_matches("file://"),
                        range.start.line + 1,
                        range.start.character + 1
                    ));
                }
                out
            }
        }
        LspActionResult::References { locations } => {
            if locations.is_empty() {
                "No references found".to_string()
            } else {
                let mut out = format!("Found {} reference(s):\n\n", locations.len());
                for loc in locations {
                    let uri = loc.uri;
                    let range = loc.range;
                    out.push_str(&format!(
                        "  {}:{}:{}\n",
                        uri.trim_start_matches("file://"),
                        range.start.line + 1,
                        range.start.character + 1
                    ));
                }
                out
            }
        }
        LspActionResult::Hover { contents, range } => {
            let mut out = "Hover information:\n\n".to_string();
            out.push_str(&contents);
            if let Some(r) = range {
                out.push_str(&format!(
                    "\n\nRange: line {}-{}, col {}-{}",
                    r.start.line + 1,
                    r.end.line + 1,
                    r.start.character + 1,
                    r.end.character + 1
                ));
            }
            out
        }
        LspActionResult::DocumentSymbols { symbols } => {
            if symbols.is_empty() {
                "No symbols found in document".to_string()
            } else {
                let mut out = format!("Document symbols ({}):\n\n", symbols.len());
                for sym in symbols {
                    out.push_str(&format!("  {} [{}]", sym.name, sym.kind));
                    if let Some(detail) = sym.detail {
                        out.push_str(&format!(" - {}", detail));
                    }
                    out.push('\n');
                }
                out
            }
        }
        LspActionResult::WorkspaceSymbols { symbols } => {
            if symbols.is_empty() {
                "No symbols found matching query".to_string()
            } else {
                let mut out = format!("Workspace symbols ({}):\n\n", symbols.len());
                for sym in symbols {
                    out.push_str(&format!("  {} [{}]", sym.name, sym.kind));
                    if let Some(uri) = sym.uri {
                        out.push_str(&format!(" - {}", uri.trim_start_matches("file://")));
                    }
                    out.push('\n');
                }
                out
            }
        }
        LspActionResult::Implementation { locations } => {
            if locations.is_empty() {
                "No implementations found".to_string()
            } else {
                let mut out = format!("Found {} implementation(s):\n\n", locations.len());
                for loc in locations {
                    let uri = loc.uri;
                    let range = loc.range;
                    out.push_str(&format!(
                        "  {}:{}:{}\n",
                        uri.trim_start_matches("file://"),
                        range.start.line + 1,
                        range.start.character + 1
                    ));
                }
                out
            }
        }
        LspActionResult::Completion { items } => {
            if items.is_empty() {
                "No completions available".to_string()
            } else {
                let mut out = format!("Completions ({}):\n\n", items.len());
                for item in items {
                    out.push_str(&format!("  {}", item.label));
                    if let Some(kind) = item.kind {
                        out.push_str(&format!(" [{}]", kind));
                    }
                    if let Some(detail) = item.detail {
                        out.push_str(&format!(" - {}", detail));
                    }
                    out.push('\n');
                }
                out
            }
        }
        LspActionResult::Diagnostics { diagnostics } => {
            if diagnostics.is_empty() {
                "No diagnostics found".to_string()
            } else {
                let mut out = format!("Diagnostics ({})\n\n", diagnostics.len());
                for diagnostic in diagnostics {
                    out.push_str(&format!(
                        "  [{}] {}:{}:{}",
                        diagnostic.severity.as_deref().unwrap_or("unknown"),
                        diagnostic.uri.trim_start_matches("file://"),
                        diagnostic.range.start.line + 1,
                        diagnostic.range.start.character + 1,
                    ));
                    if let Some(source) = diagnostic.source {
                        out.push_str(&format!(" [{source}]"));
                    }
                    if let Some(code) = diagnostic.code {
                        out.push_str(&format!(" ({code})"));
                    }
                    out.push_str(&format!("\n    {}\n", diagnostic.message));
                }
                out
            }
        }
        LspActionResult::Error { message } => {
            return Ok(ToolResult::error(message));
        }
    };

    Ok(ToolResult::success(output))
}

/// Returns `true` if the user has configured any LSP settings.
fn has_lsp_settings(settings: &crate::config::LspSettings) -> bool {
    !settings.servers.is_empty() || !settings.linters.is_empty() || settings.disable_builtin_linters
}

#[cfg(test)]
mod tests {
    use super::{LspOperation, get_file_path_arg, resolve_action_raw};
    use serde_json::json;

    #[test]
    fn parses_lsp_action_aliases() {
        assert_eq!(
            LspOperation::parse("document-symbol"),
            Some(LspOperation::DocumentSymbol)
        );
        assert_eq!(
            LspOperation::parse("workspace_symbol"),
            Some(LspOperation::WorkspaceSymbol)
        );
        assert_eq!(
            LspOperation::parse("goToImplementation"),
            Some(LspOperation::GoToImplementation)
        );
        assert_eq!(
            LspOperation::parse("diagnostics"),
            Some(LspOperation::Diagnostics)
        );
        assert_eq!(LspOperation::parse("unknown"), None);
    }

    #[test]
    fn accepts_path_alias_for_file_path() {
        let args_with_file_path = json!({
            "action": "documentSymbol",
            "file_path": "src/main.rs"
        });
        assert_eq!(get_file_path_arg(&args_with_file_path), Some("src/main.rs"));

        let args_with_path = json!({
            "action": "document-symbol",
            "path": "src/lib.rs"
        });
        assert_eq!(get_file_path_arg(&args_with_path), Some("src/lib.rs"));
    }

    #[test]
    fn maps_command_aliases_to_actions() {
        let args = json!({
            "command": "textDocument/hover",
            "file_path": "src/lib.rs",
            "line": 1,
            "column": 1
        });
        let action = resolve_action_raw(&args).expect("command should map");
        assert_eq!(action, "hover");
    }

    #[test]
    fn rejects_unsupported_command_with_helpful_error() {
        let args = json!({
            "command": "workspace/diagnostics"
        });
        let err = resolve_action_raw(&args).expect_err("unsupported command should error");
        assert!(err.to_string().contains("Unsupported lsp command"));
    }
}
