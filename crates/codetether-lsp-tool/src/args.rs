//! Resolve action and file-path aliases.
use anyhow::Result;
use serde_json::Value;

pub(super) fn get_file_path_arg(args: &Value) -> Option<&str> {
    args["file_path"].as_str().or_else(|| args["path"].as_str())
}

pub(super) fn action_from_command(command: &str) -> Option<&'static str> {
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

pub(super) fn resolve_action_raw(args: &Value) -> Result<String> {
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
