//! Tool schema for rlm_ast_query.
use codetether_provider_core::ToolDefinition;
pub(super) fn definition() -> ToolDefinition {
    ToolDefinition {
            name: "rlm_ast_query".to_string(),
            description: "Execute a tree-sitter AST query on the loaded context. Use this for structural code analysis (function signatures, struct fields, impl blocks).".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Tree-sitter S-expression query (e.g., '(function_item name: (identifier) @name)')"
                    }
                },
                "required": ["query"]
            }),
        }
}
