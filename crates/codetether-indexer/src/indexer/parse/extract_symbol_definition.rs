//! extract symbol definition for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_symbol_definition(
    language: &str,
    line: &str,
) -> Option<(&'static str, String)> {
    match language {
        "rust" => extract_rust_symbol_definition(line),
        "python" => extract_python_symbol_definition(line),
        "typescript" | "javascript" => extract_script_symbol_definition(line),
        "go" => extract_go_symbol_definition(line),
        _ => None,
    }
}
