//! extract go symbol definition for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_go_symbol_definition(
    line: &str,
) -> Option<(&'static str, String)> {
    if let Some(name) = extract_identifier_after_keyword(line, "func") {
        return Some(("function", name));
    }
    if let Some(name) = extract_identifier_after_keyword(line, "type") {
        return Some(("type", name));
    }
    if let Some(name) = extract_identifier_after_keyword(line, "const") {
        return Some(("const", name));
    }
    if let Some(name) = extract_identifier_after_keyword(line, "var") {
        return Some(("variable", name));
    }
    None
}
