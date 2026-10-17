//! extract rust symbol definition for codebase indexing.

use super::*;

pub(in crate::indexer) fn extract_rust_symbol_definition(
    line: &str,
) -> Option<(&'static str, String)> {
    let normalized = strip_prefixes(
        line,
        &[
            "pub(crate) ",
            "pub(super) ",
            "pub(self) ",
            "pub ",
            "async ",
            "unsafe ",
        ],
    );

    for (keyword, kind) in [
        ("fn", "function"),
        ("struct", "struct"),
        ("enum", "enum"),
        ("trait", "trait"),
        ("mod", "module"),
        ("type", "type"),
        ("const", "const"),
        ("static", "static"),
    ] {
        if let Some(name) = extract_identifier_after_keyword(normalized, keyword) {
            return Some((kind, name));
        }
    }

    None
}
