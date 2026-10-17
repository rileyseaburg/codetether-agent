//! Supported action spellings in the tool schema.
use serde_json::{Value, json};
pub(super) fn action() -> Value {
    json!({
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
    })
}
