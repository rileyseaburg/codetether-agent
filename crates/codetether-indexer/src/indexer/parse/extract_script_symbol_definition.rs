//! extract script symbol definition for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_script_symbol_definition(
    line: &str,
) -> Option<(&'static str, String)> {
    let normalized = strip_prefixes(line, &["export default ", "export ", "default ", "async "]);

    for (keyword, kind) in [
        ("function", "function"),
        ("class", "class"),
        ("interface", "interface"),
        ("type", "type"),
        ("enum", "enum"),
    ] {
        if let Some(name) = extract_identifier_after_keyword(normalized, keyword) {
            return Some((kind, name));
        }
    }

    for keyword in ["const", "let", "var"] {
        if let Some(name) = extract_identifier_after_keyword(normalized, keyword)
            && (normalized.contains("=>") || normalized.contains("function("))
        {
            return Some(("variable", name));
        }
    }

    None
}
