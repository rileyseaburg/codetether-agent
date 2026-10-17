//! extract python symbol definition for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_python_symbol_definition(
    line: &str,
) -> Option<(&'static str, String)> {
    let normalized = strip_prefixes(line, &["async "]);
    if let Some(name) = extract_identifier_after_keyword(normalized, "def") {
        return Some(("function", name));
    }
    if let Some(name) = extract_identifier_after_keyword(normalized, "class") {
        return Some(("class", name));
    }
    None
}
