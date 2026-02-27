//! LSP tool: Language Server Protocol operations

use crate::lsp::{LspActionResult, LspManager, detect_language_from_path};

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Global LSP manager - lazily initialized
static LSP_MANAGER: std::sync::OnceLock<Arc<RwLock<Option<Arc<LspManager>>>>> =
    std::sync::OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LspOperation {
    GoToDefinition,
    FindReferences,
    Hover,
    DocumentSymbol,
    WorkspaceSymbol,
    GoToImplementation,
    Completion,
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
            Self::DocumentSymbol | Self::WorkspaceSymbol => false,
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
        }
    }
}

fn get_file_path_arg(args: &Value) -> Option<&str> {
    args["file_path"].as_str().or_else(|| args["path"].as_str())
}

/// LSP Tool for performing Language Server Protocol operations
pub struct LspTool {
    root_uri: Option<String>,
}

impl LspTool {
    pub fn new() -> Self {
        Self { root_uri: None }
    }

    /// Create with a specific workspace root
    pub fn with_root(root_uri: String) -> Self {
        Self {
            root_uri: Some(root_uri),
        }
    }

    /// Shutdown all LSP clients, releasing resources.
    pub async fn shutdown_all(&self) {
        let cell = LSP_MANAGER.get_or_init(|| Arc::new(RwLock::new(None)));
        let guard = cell.read().await;
        if let Some(manager) = guard.as_ref() {
            manager.shutdown_all().await;
        }
    }

    /// Get or initialize the LSP manager
    async fn get_manager(&self) -> Arc<LspManager> {
        let cell = LSP_MANAGER.get_or_init(|| Arc::new(RwLock::new(None)));
        let mut guard = cell.write().await;

        if guard.is_none() {
            *guard = Some(Arc::new(LspManager::new(self.root_uri.clone())));
        }

        Arc::clone(guard.as_ref().unwrap())
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
        "Perform Language Server Protocol (LSP) operations such as go-to-definition, find-references, hover, document-symbol, workspace-symbol, and more. This tool enables AI agents to query language servers for code intelligence features. Supports rust-analyzer, typescript-language-server, pylsp, gopls, and clangd."
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
                        "completion"
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
        let action_raw = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("action is required"))?;
        let action = LspOperation::parse(action_raw)
            .ok_or_else(|| anyhow::anyhow!("Unknown action: {}", action_raw))?;

        // Get the LSP manager and client
        let manager = self.get_manager().await;

        // For workspace symbol search, we don't need a file
        if action == LspOperation::WorkspaceSymbol {
            let query = args["query"].as_str().unwrap_or("");
            let language = get_file_path_arg(&args).and_then(detect_language_from_path);

            let client = if let Some(lang) = language {
                manager.get_client(lang).await?
            } else {
                // Default to rust if we can't detect
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
                    .go_to_definition(path, line.unwrap(), column.unwrap())
                    .await?
            }
            LspOperation::FindReferences => {
                let include_decl = args["include_declaration"].as_bool().unwrap_or(true);
                client
                    .find_references(path, line.unwrap(), column.unwrap(), include_decl)
                    .await?
            }
            LspOperation::Hover => client.hover(path, line.unwrap(), column.unwrap()).await?,
            LspOperation::DocumentSymbol => client.document_symbols(path).await?,
            LspOperation::GoToImplementation => {
                client
                    .go_to_implementation(path, line.unwrap(), column.unwrap())
                    .await?
            }
            LspOperation::Completion => {
                client
                    .completion(path, line.unwrap(), column.unwrap())
                    .await?
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
        LspActionResult::Error { message } => {
            return Ok(ToolResult::error(message));
        }
    };

    Ok(ToolResult::success(output))
}

#[cfg(test)]
mod tests {
    use super::{LspOperation, get_file_path_arg};
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
}
